import http.server
import ssl
import json
import subprocess
import json
import os
from multiprocessing import Process, Value, Manager
import time

is_busy = Value("i", 0)  # 0: idle, 1: busy
manager = Manager()
tasks = manager.dict()
start = time.perf_counter()


class SimpleHTTPSRequestHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header("Access-Control-Allow-Origin", "*")
        self.send_header("Access-Control-Allow-Methods", "GET,POST,OPTIONS")
        self.send_header("Access-Control-Allow-Headers", "*")
        # self.send_header("Access-Control-Allow-Credentials", "true")  # If credentials (cookies) are needed
        self.send_header("Referrer-Policy", "strict-origin-when-cross-origin")
        super().end_headers()

    def do_OPTIONS(self):  # Handle preflight requests
        self.send_response(200, "OK")
        self.end_headers()

    def run_command(self, requestid, attestationData):
        t_start = time.perf_counter()
        try:
            input_dir = f"./request_data"
            output_dir = f"./proof_output/{requestid}"
            os.makedirs(input_dir, exist_ok=True)
            os.makedirs(output_dir, exist_ok=True)

            input_file = f"{input_dir}/{requestid}.json"
            with open(input_file, "w", encoding="utf-8") as f:
                f.write(attestationData)

            cmd = [
                "../target/release/zktls-prover",
                "--elf",
                "../zktls/app/elf/riscv32im-pico-zkvm-elf",
                "--input",
                input_file,
                "--config",
                "../zktls/prover/data/attestation_config_prod.json",  # dev/test: attestation_config.json
                "--output-dir",
                output_dir,
            ]
            print("[CMD]", cmd)
            result = subprocess.run(cmd, capture_output=True, text=True)
            print("[OUTPUT]:", result.stdout)
            if result.stderr:
                print("[ERROR]:", result.stderr)

            pv_file = ""
            if os.path.exists(f"{output_dir}/pv_file"):
                with open(f"{output_dir}/pv_file", "r", encoding="utf-8") as f:
                    pv_file = f.read()

            proof = ""
            if os.path.exists(f"{output_dir}/proof.json"):
                with open(f"{output_dir}/proof.json", "r", encoding="utf-8") as f:
                    proof = f.read()

            t_end = time.perf_counter()
            tasks[requestid] = {
                "status": "done",
                "returncode": result.returncode,
                "stdout": result.stdout,
                "stderr": result.stderr,
                "pv_file": pv_file,
                "proof": proof,
                "elapsed": f"{t_end - t_start:.6f}",
            }
            print(f"[ELAPSED]: {t_end - t_start:.6f}")
        except Exception as e:
            print("[EXCEPTION]:", str(e))
            t_end = time.perf_counter()
            tasks[requestid] = {
                "status": "error",
                "returncode": -1,
                "stdout": "",
                "stderr": str(e),
                "pv_file": "",
                "proof": "",
                "elapsed": f"{t_end - t_start:.6f}",
            }
            print(f"[ELAPSED]: {t_end - t_start:.6f}")
        finally:
            is_busy.value = 0

    def do_POST(self):
        if self.path not in ["/zktls/prove", "/zktls/result"]:
            data = {"code": "10001", "description": "only support /zktls/prove, /zktls/result"}
            self.send_response(404)
            self.send_header("Content-type", "application/json")
            self.end_headers()
            self.wfile.write(json.dumps(data, ensure_ascii=False).encode("utf-8"))
            return

        content_length = int(self.headers.get("Content-Length", 0))
        body = self.rfile.read(content_length).decode("utf-8")
        print("body", body)

        data = json.loads(body)
        requestid = data["requestid"]

        if self.path == "/zktls/prove":
            if is_busy.value == 1:
                data = {"code": "10002", "description": "Server is busy, please try later."}
                self.send_response(200)
                self.send_header("Content-type", "application/json")
                self.end_headers()
                self.wfile.write(json.dumps(data, ensure_ascii=False).encode("utf-8"))
                return

            # the body is json string
            attestationData = json.dumps(data["attestationData"], separators=(",", ":"), ensure_ascii=False)
            print("requestid", requestid)
            print("attestationData", attestationData)

            # set status
            is_busy.value = 1
            tasks[requestid] = {"status": "running"}

            # execute prove program
            Process(target=self.run_command, args=(requestid, attestationData)).start()

            # response
            data = {"code": "0", "description": "success"}
            self.send_response(200)
            self.send_header("Content-type", "application/json")
            self.end_headers()
            self.wfile.write(json.dumps(data, ensure_ascii=False).encode("utf-8"))
        elif self.path == "/zktls/result":
            task = tasks.get(requestid)
            if not task:
                data = {"code": "10003", "description": f"requestid {requestid} not exist!"}
                self.send_response(200)
                self.send_header("Content-type", "application/json")
                self.end_headers()
                self.wfile.write(json.dumps(data, ensure_ascii=False).encode("utf-8"))
                return

            data = {
                "code": "0",
                "description": "success",
                "details": task,
            }
            self.send_response(200)
            self.send_header("Content-type", "application/json")
            self.end_headers()
            self.wfile.write(json.dumps(data, ensure_ascii=False).encode("utf-8"))


useSSL = False
port = 38080
httpd = http.server.HTTPServer(("0.0.0.0", port), SimpleHTTPSRequestHandler)

if useSSL:
    context = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
    # openssl req -x509 -newkey rsa:4096 -nodes -out cert.pem -keyout key.pem -days 365
    context.load_cert_chain(certfile="cert.pem", keyfile="key.pem")
    httpd.socket = context.wrap_socket(httpd.socket, server_side=True)
    print(f"Serving HTTPS on 0.0.0.0 port {port} (https://127.0.0.1:{port}/) ...")
else:
    print(f"Serving HTTP on 0.0.0.0 port {port} (http://127.0.0.1:{port}/) ...")
httpd.serve_forever()

# curl -X POST http://127.0.0.1:38080/zktls/prove -H "Content-Type: application/json" -d '{"requestid": "1234","attestationData": {"public_data": {},"private_data": {}}}'
# curl -X POST http://127.0.0.1:38080/zktls/result -H "Content-Type: application/json" -d '{"requestid": "1234"}'
