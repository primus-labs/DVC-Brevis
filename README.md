## DVC-Brevis (ZKTLS Integration)

This example demonstrates how to verify diverse Web2 data sources by leveraging the synergy between **Primus zkTLS** and **Brevis zkVM**.

The end-to-end workflow follows a two-stage process:

1. **Data Attestation (Primus zkTLS):** On the client side, a browser extension uses zkTLS to generate cryptographic proofs of specific Web2 data items, ensuring data authenticity and privacy at the source.
2. **Computation & Verification (Brevis zkVM):** These raw data items, along with the hashed version, are then fed into the Brevis zkVM, which executes custom business logic (such as threshold checks, aggregations, or complex calculations) to generate a final verification. 

## Data Sources and Format

### 1. CEXs (Binance, OKX)

- in progress

### 2. Github

- **Description**: Verifies a user's GitHub user id, contribution history in the last year, and account longevity.

- **Verified Fields (`data` object)**:

  - `github_id_in_html`: The `profile_user_id` extracted from the HTML metadata of the profile being viewed.

  - `github_id`: The unique numerical identifier extracted from the `avatarUrl`. This represents the **authenticated account** currently logged in.

    > **Security Note:** The zkVM should compare `github_id` with `github_id_in_html`. If they do not match, it indicates the user is attempting to generate a proof using someone else's profile page. In this case, the verification session is invalidated to ensure data ownership.

  - `contribution`: A verified string of total contributions in the last year (e.g., `"1,040 contributions"`).

  - `years`: A verified array of all years since account creation (e.g., `["2026", "2025", ...]`).

- **Attestation data in plaintext**

```
"private_data": [
  {
    "id": "github_id_in_html",
    "content": "[\"{&quot;event_type&quot;:&quot;user_profile.click&quot;,&quot;payload&quot;:{&quot;profile_user_id&quot;:34767505,&quot;target&quot;:&quot;CONTRIBUTION_YEAR_LINK&quot;,&quot;user_id&quot;:8588607,&quot;originating_url&quot;:&quot;https://github.com/xudean?action=show&amp;controller=profiles&amp;tab=contributions&amp;user_id=xudean&quot;}}\"]"
  },
  {
    "id": "contribution",
    "content": "[\"1,040\\n      contributions\\n        in the last year\"]"
  },
  {
    "id": "github_id",
    "content": "[\"https://avatars.githubusercontent.com/u/8588607?v=4\"]"
  },
  {
    "id": "years",
    "content": "\"2026\",\"2025\",\"2024\",\"2023\",\"2022\",\"2021\",\"2020\",\"2019\",\"2018\",\"2017\"]"
  }
]
```

### 3. Steam

- in progress

### 4. Amazon

- in progress


## Technical Data Structure

All data passed to the zkVM is in a format like this:

- https://github.com/primus-labs/DVC-Brevis/blob/brevis/zktls/prover/data/binance_kyc_status.json

The main difference between data sources lies in the content of the `private_data` field.

The `private_data` field contains the plaintext data returned by the data source. The zkVM needs to extract this plaintext data and then perform the corresponding business logic computations.

For the meaning of the specific data fields, please refer to the [Data Sources and Format](https://github.com/primus-labs/DVC-Brevis/tree/brevis?tab=readme-ov-file#data-sources-and-format) section.


## Integration Guide

### Prerequisite

1. Install [Pico toolchains](https://pico-docs.brevis.network/getting-started/installation).

### Build

```sh
cd zktls/app
RUST_LOG=info cargo pico build

cd zktls/prover
RUST_LOG=info cargo build --release
```

### Run

```sh
cd zktls/prover
RUST_LOG=info cargo run --release
```

### localtest

```sh
bash ./build.sh
RUST_LOG=info ./target/release/zktls-prover \
  --elf ./zktls/app/elf/riscv32im-pico-zkvm-elf \
  --input ./zktls/prover/data/attestation_data.json \
  --output-dir pico_out
```
