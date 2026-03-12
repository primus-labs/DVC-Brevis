## DVC-Brevis (ZKTLS Integration)

This example demonstrates how to verify diverse Web2 data sources by leveraging the synergy between **Primus zkTLS** and **Brevis zkVM**.

The end-to-end workflow follows a two-stage process:

1. **Data Attestation (Primus zkTLS):** On the client side, a browser extension uses zkTLS to generate cryptographic proofs of specific Web2 data items, ensuring data authenticity and privacy at the source.
2. **Computation & Verification (Brevis zkVM):** These raw data items, along with the hashed version, are then fed into the Brevis zkVM, which executes custom business logic (such as threshold checks, aggregations, or complex calculations) to generate a final verification. 

## Data Sources and Format

### 1. CEXs (Binance, OKX)

#### Binance

- **Description**: Verifies a user's Binance user ID, KYC level, and transaction history list.

- **Verified Fields (`data` object)**:
  - `userId`: The unique identifier for the Binance account.
  - `passKycLevel`: The current KYC level of the user (e.g., `"INTERMEDIATE"`).
  - `list`: A verified list of asset transaction history in the past 6 months.

### 2. Github

- **Description**: Verifies a user's GitHub user ID, contribution history in the last year, and account longevity.

- **Verified Fields (`data` object)**:

  - `github_id_in_html`: The `profile_user_id` extracted from the HTML metadata of the profile being viewed (e.g., `34767505`).

  - `github_id`: The unique numerical identifier extracted from the `avatarUrl` (e.g., `8588607`). This represents the **authenticated account** currently logged in.

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

- **Description**: Verifies a user's Steam identity and historical transaction data (including game purchases, date, spending, and refunds).

- **Verified Fields (`data` object)**:

  - `profile_info`: The user's unique Steam ID contained within the `href` attribute of the profile link. (e.g., `76561198382985081`)

  - `purchase_history`: The verified raw table containing transaction dates, item names, and total prices.
  
    > **Note 1:** The zkVM parses the purchase history table to calculate the total net spend (purchase price - refund price) and can verify two conditions:
    > 
    > 1.**Limited Account Check**: If the total spend is less than $5, the account is flagged as a Limited Account.
    > 
    > 2.**Game Library Value**: Determines if the total value of games purchased exceeds $50.
    
    > **Note 2:** The zkVM identifies the earliest transaction date in the purchase history to determine the account's creation date and calculate the account age.

    <p align="center">
    <img src="https://github.com/user-attachments/assets/39acffb2-a99c-462a-b896-9e794092a7b7" width="70%" alt="steam-purchase-history" />
    <br>
    <em>Visual Reference: Historical transaction data as seen by the user.</em>
    </p> 


- **Attestation data in plaintext**

```
"data": "{\"purchase_history\":\"<div id=\\\"main_content\\\" class=\\\"page_content\\\" >\\n\\t\\t\\t\\t\\t\\t<div class=\\\"wallet_history_click_hint\\\">\\n\\t\\t\\tProblem with a transaction? Select it below to get help.\\t\\t</div>\\n\\t\\t<table class=\\\"wallet_history_table\\\">\\n\\t\\t\\t<thead>\\n\\t\\t\\t\\t<tr>\\n\\t\\t\\t\\t\\t<th rowspan=\\\"2\\\" class=\\\"wht_date\\\">Date</th>\\n\\t\\t\\t\\t\\t<th rowspan=\\\"2\\\" class=\\\"wht_items\\\">Items</th>\\n\\t\\t\\t\\t\\t<th rowspan=\\\"2\\\" class=\\\"wht_type\\\">Type</th>\\n\\t\\t\\t\\t\\t<th rowspan=\\\"2\\\" class=\\\"wht_baseprice\\\">Price</th>\\n\\t\\t\\t\\t\\t<th rowspan=\\\"2\\\" class=\\\"wht_tax\\\">Tax</th>\\n\\t\\t\\t\\t\\t<th rowspan=\\\"2\\\" class=\\\"wht_shipping\\\">Shipping</th>\\n\\t\\t\\t\\t\\t<th rowspan=\\\"2\\\" class=\\\"wht_total\\\">Total</th>\\n\\t\\t\\t\\t\\t<th class=\\\"wht_wallet\\\" colspan=\\\"2\\\">Wallet</th>\\n\\t\\t\\t\\t</tr>\\n\\t\\t\\t\\t<tr>\\n\\t\\t\\t\\t\\t<th class=\\\"wht_wallet_change\\\">Change</th>\\n\\t\\t\\t\\t\\t<th class=\\\"wht_wallet_balance\\\">Balance</th>\\n\\t\\t\\t\\t</tr>\\n\\t\\t\\t</thead>\\n\\t\\t\\t<tbody>\\n\\t\\t\\t\\t\\t\\t<tr data-panel=\\\"{&quot;focusable&quot;:true,&quot;clickOnActivate&quot;:true}\\\" role=\\\"button\\\" class=\\\"wallet_table_row \\\" onclick=\\\"location.href='https://help.steampowered.com/en/wizard/HelpWithTransaction?transid=4284700772204049485'\\\">\\n\\t\\t<td class=\\\"wht_date\\\">15 Sep, 2024</td>\\n\\t\\t<td data-tooltip-text=\\\"Click to get help with this purchase\\\" class=\\\"wht_items \\\"\\\">\\n\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t<div style=\\\"clear: both\\\">\\n\\t\\t\\t\\t\\t\\tChivalry 2\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t</div>\\n\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t<div class=\\\"wth_item_refunded\\\">\\n\\t\\t\\t\\t\\t\\tRefund\\t\\t\\t\\t\\t</div>\\n\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t</td>\\n\\t\\t<td class=\\\"wht_type \\\">\\n\\t\\t\\t\\t\\t\\t\\t\\t\\t<div>Refund</div>\\n\\t\\t\\t\\t\\t\\t<div class=\\\"wth_payment\\\">\\n\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\tAliPay\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t</div>\\n\\t\\t\\t\\t</td>\\n\\t\\t<td class=\\\"wht_base_price\\\">\\n\\t\\t\\t\\t\\t</td>\\n\\t\\t<td class=\\\"wht_tax\\\">\\n\\t\\t\\t\\t\\t</td>\\n\\t\\t<td class=\\\"wht_shipping\\\">\\n\\t\\t\\t\\t\\t</td>\\n\\t\\t<td class=\\\"wht_total \\\">\\n\\t\\t\\t\\t\\t\\t\\t¥ 116.00\\t\\t\\t\\t\\t</td>\\n\\n\\t\\t\\t\\t\\t<td class=\\\"wht_wallet_change\\\"></td>\\n\\t\\t\\t<td class=\\\"wht_wallet_balance\\\"></td>\\n\\t\\t\\t</tr>\\n\\t\\t<tr data-panel=\\\"{&quot;focusable&quot;:true,&quot;clickOnActivate&quot;:true}\\\" role=\\\"button\\\" class=\\\"wallet_table_row \\\" onclick=\\\"location.href='https://help.steampowered.com/en/wizard/HelpWithTransaction?transid=4284700772204049485'\\\">\\n\\t\\t<td class=\\\"wht_date\\\">15 Sep, 2024</td>\\n\\t\\t<td data-tooltip-text=\\\"Click to get help with this purchase\\\" class=\\\"wht_items wht_item_refunded\\\"\\\">\\n\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t<div style=\\\"clear: both\\\">\\n\\t\\t\\t\\t\\t\\tChivalry 2\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t</div>\\n\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t</td>\\n\\t\\t<td class=\\\"wht_type wht_refunded\\\">\\n\\t\\t\\t\\t\\t\\t\\t\\t\\t<div>Purchase</div>\\n\\t\\t\\t\\t\\t\\t<div class=\\\"wth_payment\\\">\\n\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\tAliPay\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t</div>\\n\\t\\t\\t\\t</td>\\n\\t\\t<td class=\\\"wht_base_price\\\">\\n\\t\\t\\t¥ 116.00\\t\\t</td>\\n\\t\\t<td class=\\\"wht_tax\\\">\\n\\t\\t\\t\\t\\t</td>\\n\\t\\t<td class=\\\"wht_shipping\\\">\\n\\t\\t\\t\\t\\t</td>\\n\\t\\t<td class=\\\"wht_total wht_refunded\\\">\\n\\t\\t\\t\\t\\t\\t\\t¥ 116.00\\t\\t\\t\\t\\t</td>\\n\\n\\t\\t\\t\\t\\t<td class=\\\"wht_wallet_change\\\"></td>\\n\\t\\t\\t<td class=\\\"wht_wallet_balance\\\"></td>\\n\\t\\t\\t</tr>\\n\\t\\t\\t<tr data-panel=\\\"{&quot;focusable&quot;:true,&quot;clickOnActivate&quot;:true}\\\" role=\\\"button\\\" class=\\\"wallet_table_row \\\" onclick=\\\"location.href='https://help.steampowered.com/en/wizard/HelpWithTransaction?transid=1478797762232260746'\\\">\\n\\t\\t<td class=\\\"wht_date\\\">6 Nov, 2019</td>\\n\\t\\t<td data-tooltip-text=\\\"Click to get help with this purchase\\\" class=\\\"wht_items \\\"\\\">\\n\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t<div style=\\\"clear: both\\\">\\n\\t\\t\\t\\t\\t\\tWallpaper Engine\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t</div>\\n\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t</td>\\n\\t\\t<td class=\\\"wht_type \\\">\\n\\t\\t\\t\\t\\t\\t\\t\\t\\t<div>Purchase</div>\\n\\t\\t\\t\\t\\t\\t<div class=\\\"wth_payment\\\">\\n\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\tAliPay\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t</div>\\n\\t\\t\\t\\t</td>\\n\\t\\t<td class=\\\"wht_base_price\\\">\\n\\t\\t\\t¥ 19.00\\t\\t</td>\\n\\t\\t<td class=\\\"wht_tax\\\">\\n\\t\\t\\t\\t\\t</td>\\n\\t\\t<td class=\\\"wht_shipping\\\">\\n\\t\\t\\t\\t\\t</td>\\n\\t\\t<td class=\\\"wht_total \\\">\\n\\t\\t\\t\\t\\t\\t\\t¥ 19.00\\t\\t\\t\\t\\t</td>\\n\\n\\t\\t\\t\\t\\t<td class=\\\"wht_wallet_change\\\"></td>\\n\\t\\t\\t<td class=\\\"wht_wallet_balance\\\"></td>\\n\\t\\t\\t</tr>\\n\\t\\t<tr data-panel=\\\"{&quot;focusable&quot;:true,&quot;clickOnActivate&quot;:true}\\\" role=\\\"button\\\" class=\\\"wallet_table_row \\\" onclick=\\\"location.href='https://help.steampowered.com/en/wizard/HelpWithTransaction?transid=1175846430789332971'\\\">\\n\\t\\t<td class=\\\"wht_date\\\">24 Apr, 2017</td>\\n\\t\\t<td data-tooltip-text=\\\"Click to get help with this purchase\\\" class=\\\"wht_items \\\"\\\">\\n\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t<div style=\\\"clear: both\\\">\\n\\t\\t\\t\\t\\t\\tPLAYERUNKNOWN'S BATTLEGROUNDS - Worldwide Package\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t</div>\\n\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t</td>\\n\\t\\t<td class=\\\"wht_type \\\">\\n\\t\\t\\t\\t\\t\\t\\t\\t\\t<div>Purchase</div>\\n\\t\\t\\t\\t\\t\\t<div class=\\\"wth_payment\\\">\\n\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\tAliPay\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t</div>\\n\\t\\t\\t\\t</td>\\n\\t\\t<td class=\\\"wht_base_price\\\">\\n\\t\\t\\t¥ 98.00\\t\\t</td>\\n\\t\\t<td class=\\\"wht_tax\\\">\\n\\t\\t\\t\\t\\t</td>\\n\\t\\t<td class=\\\"wht_shipping\\\">\\n\\t\\t\\t\\t\\t</td>\\n\\t\\t<td class=\\\"wht_total \\\">\\n\\t\\t\\t\\t\\t\\t\\t¥ 98.00\\t\\t\\t\\t\\t</td>\\n\\n\\t\\t\\t\\t\\t<td class=\\\"wht_wallet_change\\\"></td>\\n\\t\\t\\t<td class=\\\"wht_wallet_balance\\\"></td>\\n\\t\\t\\t</tr>\\n\\t\\t\\t\\t<tr id=\\\"more_history\\\" style=\\\"display: none\\\"></tr>\\n\\t\\t\\t</tbody>\\n\\t\\t</table>\\n\\n\\t\\t<div class=\\\"load_more_history_area\\\">\\n\\t\\t\\t<div data-panel=\\\"{&quot;focusable&quot;:true,&quot;clickOnActivate&quot;:true}\\\" role=\\\"button\\\" id=\\\"load_more_button\\\" class=\\\"btnv6_blue_hoverfade btn_medium\\\" onclick=\\\"WalletHistory_LoadMore(); return false;\\\" style=\\\"display:none\\\" >\\n\\t\\t\\t\\tLoad More Transactions\\t\\t\\t</div>\\n\\t\\t\\t<div id=\\\"wallet_history_loading\\\" style=\\\"display: none; \\\">\\n\\t\\t\\t\\t<img src=\\\"https://store.akamai.steamstatic.com/public/images/login/throbber.gif\\\">\\n\\t\\t\\t</div>\\n\\t\\t</div>\\n\\t</div>\",\"profile_info\":\"<a href=\\\"https://steamcommunity.com/profiles/76561198382985081/\\\" aria-label=\\\"View your profile\\\">\\n\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t<img src=\\\"https://avatars.akamai.steamstatic.com/fef49e7fa7e1997310d705b2a6158ff8dc1cdfeb_full.jpg\\\" alt=\\\"安澜公子\\\">\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t\\t</a>\"}",
```



### 4. Amazon

- in progress


## Technical Data Structure

All attestation inputs sent to the zkVM follow the same top-level JSON shape. A concrete example is:

- [`zktls/prover/data/binance_kyc_status.json`](https://github.com/primus-labs/DVC-Brevis/blob/brevis/zktls/prover/data/binance_kyc_status.json)

At the top level, the payload contains:

- `verification_type`: Verification mode used by Primus zkTLS, for example `HASH_COMPARISON`.
- `public_data`: The attestation generated by zkTLS, including the hash of the plaintext response returned by the data source.
- `private_data`: The plaintext response returned by the corresponding data source, used by the zkVM for business logic computation. For the business meaning of each source-specific field, refer to the [Data Sources and Format](https://github.com/primus-labs/DVC-Brevis/tree/brevis?tab=readme-ov-file#data-sources-and-format) section.

The zkVM logic in [`zktls/app/src/main.rs`](https://github.com/primus-labs/DVC-Brevis/blob/brevis/zktls/app/src/main.rs) works as follows:

1. Use `verify_attestation_data` to verify the zkTLS signature, verify that the plaintext returned by the data source matches the corresponding hash, and validate the data source URL.
2. Extract `private_data` and use it for business logic computation.


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
