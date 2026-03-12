## DVC-Brevis (ZKTLS Integration)

This example demonstrates how to verify diverse Web2 data sources by leveraging the synergy between **Primus zkTLS** and **Brevis zkVM**.

The end-to-end workflow follows a two-stage process:

1. **Data Attestation (Primus zkTLS):** On the client side, a browser extension uses zkTLS to generate cryptographic proofs of specific Web2 data items, ensuring data authenticity and privacy at the source.
2. **Computation & Verification (Brevis zkVM):** These raw data items, along with the hashed version, are then fed into the Brevis zkVM, which executes custom business logic (such as threshold checks, aggregations, or complex calculations) to generate a final verification. 

## Data Sources and Format

### 1. CEXs (Binance, OKX)

#### Binance

- **Description**: Verifies a user's Binance user ID, KYC level, and transaction history list.

- **zkTLS Verified Fields (`data` object)**:
  - `userId`: The unique identifier for the Binance account.
  - `passKycLevel`: The current KYC level of the user (e.g., `"INTERMEDIATE"`).
  - `list`: A verified list of asset transaction history in the past 6 months.
 
- **Computation via zkVM**

  - **Account ID**: Parses the `userId` (e.g., 782151446). *(zkVM Public: Hash)*

  - **KYC Level Verification**: Parses the `passKycLevel` (e.g., "INTERMEDIATE") and compares it against the required KYC level provided by the business input (e.g., complete advanced verification).  *(zkVM Public: Comparison Result)*
 
  - **Transaction Activity**: Parses the `data` array to count the total number of trade entries from the spot trade history. Compares this count against the transaction threshold provided by the business input (e.g., > 50 trades). *(zkVM Public: Comparison Result)*


- **Attestation data in plaintext**

```json
{
  "private_data": [
    {
      "id": "passKycLevel",
      "salt": "c163f1aa9a1e5f33ec9c42b4c1f31ae1",
      "content": [
        "INTERMEDIATE"
      ]
    },
    {
      "id": "userId",
      "salt": "ab7a4b4c93715ad5aeb8705a6ce34e90",
      "content": [
        "782151446"
      ]
    },
    {
      "id": "data",
      "salt": "84f817877022bb1759c2fdf23f042291",
      "content": [
        "[{\"tradeId\":327619867,\"orderId\":8131702704,\"tradeIdStr\":\"327619867\",\"price\":\"64000\",\"time\":1770349700000,\"symbol\":\"BTCUSDC\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.00156\",\"fee\":\"0.00012225\",\"feeAsset\":\"BNB\",\"totalQuota\":\"99.84\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDC\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":99.84,\"usdtRate\":64000.00000000},{\"tradeId\":326778255,\"orderId\":8034497893,\"tradeIdStr\":\"326778255\",\"price\":\"66000\",\"time\":1770315414000,\"symbol\":\"BTCUSDC\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.00151\",\"fee\":\"0.00011463\",\"feeAsset\":\"BNB\",\"totalQuota\":\"99.66\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDC\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":99.66,\"usdtRate\":66000.00000000},{\"tradeId\":326549251,\"orderId\":8034492826,\"tradeIdStr\":\"326549251\",\"price\":\"68000\",\"time\":1770304376000,\"symbol\":\"BTCUSDC\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.00147\",\"fee\":\"0.00011331\",\"feeAsset\":\"BNB\",\"totalQuota\":\"99.96\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDC\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":99.96,\"usdtRate\":68000.00000000},{\"tradeId\":326353752,\"orderId\":8034469879,\"tradeIdStr\":\"326353752\",\"price\":\"70000\",\"time\":1770290677000,\"symbol\":\"BTCUSDC\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.00142\",\"fee\":\"0.00010992\",\"feeAsset\":\"BNB\",\"totalQuota\":\"99.4\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDC\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":99.40,\"usdtRate\":70000.00000000},{\"tradeId\":325804454,\"orderId\":8034466609,\"tradeIdStr\":\"325804454\",\"price\":\"72000\",\"time\":1770242749000,\"symbol\":\"BTCUSDC\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.00138\",\"fee\":\"0.00010725\",\"feeAsset\":\"BNB\",\"totalQuota\":\"99.36\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDC\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":99.36,\"usdtRate\":72000.00000000},{\"tradeId\":1228969253,\"orderId\":9468281225,\"tradeIdStr\":\"1228969253\",\"price\":\"1197.84\",\"time\":1760423768000,\"symbol\":\"BNBUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.008\",\"fee\":\"0.000006\",\"feeAsset\":\"BNB\",\"totalQuota\":\"9.58272\",\"productName\":\"\",\"baseAsset\":\"BNB\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":9.58,\"usdtRate\":1197.84000000}]"
      ]
    }
  ]
}
```



### 2. Github

- **Description**: Verifies a user's GitHub user ID, contribution history in the last year, and account longevity.

- **zkTLS Verified Fields (`data` object)**:

  - `github_id_in_html`: The `profile_user_id` extracted from the HTML metadata of the profile being viewed (e.g., `25573916`).

  - `github_id`: The unique numerical identifier extracted from the `avatarUrl` (e.g., `25573916`). This represents the **authenticated account** currently logged in.

    > **Security Note:** The zkVM should compare `github_id` with `github_id_in_html`. If they do not match, it indicates the user is attempting to generate a proof using someone else's profile page. In this case, the verification session is invalidated to ensure data ownership.

  - `contribution`: A verified string of total contributions in the last year (e.g., `"20   contributions"`).

  - `years`: A verified array of all years since account creation (e.g., `["2026", "2025", ...]`).

 
- **Computation via zkVM**

  - **Account ID**: Parses  `github_id_in_html` and  `github_id` to extract two github IDs. Performs a strict comparison between them: If they do not match, terminate the verification process with an error. If they match, set this GitHub ID as the user's unique identifier and account ID. *(zkVM Public: Hash)*

  - **Contributions Number**: Parse the  `contribution` field to extract the specific number of contributions. Compares the specific number against the threshold provided by the business input (e.g., more than 50 contributions). *(zkVM Public: Comparison Result)*
 
  - **Registration Time**: Parses the `years` array to find the earliest years as the account registration time. Compares the earliest years against the time threshold provided by the business input (e.g., registered for more than 5 years). *(zkVM Public: Comparison Result)* 

- **Attestation data in plaintext**

```json
{
  "private_data": [
    {
      "id": "contribution",
      "salt": "2bb150ea11dbf05522c3edef037b144b",
      "content": [
        "20\n      contributions\n        in the last year"
      ]
    },
    {
      "id": "github_id",
      "salt": "b363ce2f2dc4318d423e94c749c89236",
      "content": [
        "https://avatars.githubusercontent.com/u/25573916?v=4"
      ]
    },
    {
      "id": "github_id_in_html",
      "salt": "fea0ef7c912e9b39887e4d9e07c8a73e",
      "content": [
        "{&quot;event_type&quot;:&quot;user_profile.click&quot;,&quot;payload&quot;:{&quot;profile_user_id&quot;:25573916,&quot;target&quot;:&quot;CONTRIBUTION_YEAR_LINK&quot;,&quot;user_id&quot;:25573916,&quot;originating_url&quot;:&quot;https://github.com/ehisper?action=show&amp;controller=profiles&amp;tab=contributions&amp;user_id=ehisper&quot;}}"
      ]
    },
    {
      "id": "years",
      "salt": "44d23d1923b4fdab99ed9147b11d7c46",
      "content": [
        "2026",
        "2025",
        "2024",
        "2023",
        "2022",
        "2021",
        "2020",
        "2019",
        "2018",
        "2017"
      ]
    }
  ]
}
```

### 3. Steam

- **Description**: Verifies a user's Steam identity and historical transaction data (including game purchases, date, spending, and refunds).

- **zkTLS Verified Fields (`data` object)**:

  - `profile_info`: The user's unique Steam ID contained within the `href` attribute of the profile link. (e.g., `76561198382985081`)

  - `purchase_history`: The verified raw table containing transaction dates, item names, and total prices.

    <p align="center">
    <img src="https://github.com/user-attachments/assets/39acffb2-a99c-462a-b896-9e794092a7b7" width="80%" alt="steam-purchase-history" />
    <br>
    <em>Visual Reference: Historical transaction data as seen by the user.</em>
    </p> 

- **Computation via zkVM**

  - **Account ID**: Parses the `href` in `profile_info` to extract the unique 17-digit Steam ID. *(zkVM Public: Hash)*

  - **Game Library Value**: Parses the purchase history table to calculate the total net spend (purchase price - refund price) and sets this value as the game library value. Compares the value against the spending threshold provided by the business input (e.g., $50). *(zkVM Public: Comparison Result)*

  - **Limited Account Judgement**: Compare the Game Library Value to $5. If the value is less than $5, the account is flagged as a "Limited Account". *(zkVM Public: Limited Account or not)*

  - **Registration Time**: Identifies the earliest transaction date in the purchase history to determine the account's creation date and sets this as the account registration time. Compares the date against the time threshold provided by the business input (e.g., registered for more than 5 years). *(zkVM Public: Comparison Result)*

- **Attestation data in plaintext**

```json
{
  "private_data": [
    {
      "id": "profile_info",
      "salt": "e0d61d080cd02c4323fa3b7aba6ef436",
      "content": [
        "<a href=\"https://steamcommunity.com/profiles/76561198382985081/\" aria-label=\"View your profile\">\n\t\t\t\t\t\t\t\t\t\t\t\t\t<img src=\"https://avatars.fastly.steamstatic.com/fef49e7fa7e1997310d705b2a6158ff8dc1cdfeb_full.jpg\" alt=\"安澜公子\">\t\t\t\t\t\t\t\t\t\t\t\t</a>"
      ]
    },
    {
      "id": "purchase_history",
      "salt": "7e27a363fd188357fa194c3aa972663d",
      "content": [
        "<div id=\"main_content\" class=\"page_content\" >\n\t\t\t\t\t\t<div class=\"wallet_history_click_hint\">\n\t\t\tProblem with a transaction? Select it below to get help.\t\t</div>\n\t\t<table class=\"wallet_history_table\">\n\t\t\t<thead>\n\t\t\t\t<tr>\n\t\t\t\t\t<th rowspan=\"2\" class=\"wht_date\">Date</th>\n\t\t\t\t\t<th rowspan=\"2\" class=\"wht_items\">Items</th>\n\t\t\t\t\t<th rowspan=\"2\" class=\"wht_type\">Type</th>\n\t\t\t\t\t<th rowspan=\"2\" class=\"wht_baseprice\">Price</th>\n\t\t\t\t\t<th rowspan=\"2\" class=\"wht_tax\">Tax</th>\n\t\t\t\t\t<th rowspan=\"2\" class=\"wht_shipping\">Shipping</th>\n\t\t\t\t\t<th rowspan=\"2\" class=\"wht_total\">Total</th>\n\t\t\t\t\t<th class=\"wht_wallet\" colspan=\"2\">Wallet</th>\n\t\t\t\t</tr>\n\t\t\t\t<tr>\n\t\t\t\t\t<th class=\"wht_wallet_change\">Change</th>\n\t\t\t\t\t<th class=\"wht_wallet_balance\">Balance</th>\n\t\t\t\t</tr>\n\t\t\t</thead>\n\t\t\t<tbody>\n\t\t\t\t\t\t<tr data-panel=\"{&quot;focusable&quot;:true,&quot;clickOnActivate&quot;:true}\" role=\"button\" class=\"wallet_table_row \" onclick=\"location.href='https://help.steampowered.com/en/wizard/HelpWithTransaction?transid=4284700772204049485'\">\n\t\t<td class=\"wht_date\">15 Sep, 2024</td>\n\t\t<td data-tooltip-text=\"Click to get help with this purchase\" class=\"wht_items \"\">\n\t\t\t\t\t\t\t\t\t\t\t\t<div style=\"clear: both\">\n\t\t\t\t\t\tChivalry 2\t\t\t\t\t\t\t\t\t\t\t</div>\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t<div class=\"wth_item_refunded\">\n\t\t\t\t\t\tRefund\t\t\t\t\t</div>\n\t\t\t\t\t\t\t\t\t\t\t\t</td>\n\t\t<td class=\"wht_type \">\n\t\t\t\t\t\t\t\t\t<div>Refund</div>\n\t\t\t\t\t\t<div class=\"wth_payment\">\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\tAliPay\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t</div>\n\t\t\t\t</td>\n\t\t<td class=\"wht_base_price\">\n\t\t\t\t\t</td>\n\t\t<td class=\"wht_tax\">\n\t\t\t\t\t</td>\n\t\t<td class=\"wht_shipping\">\n\t\t\t\t\t</td>\n\t\t<td class=\"wht_total \">\n\t\t\t\t\t\t\t¥ 116.00\t\t\t\t\t</td>\n\n\t\t\t\t\t<td class=\"wht_wallet_change\"></td>\n\t\t\t<td class=\"wht_wallet_balance\"></td>\n\t\t\t</tr>\n\t\t<tr data-panel=\"{&quot;focusable&quot;:true,&quot;clickOnActivate&quot;:true}\" role=\"button\" class=\"wallet_table_row \" onclick=\"location.href='https://help.steampowered.com/en/wizard/HelpWithTransaction?transid=4284700772204049485'\">\n\t\t<td class=\"wht_date\">15 Sep, 2024</td>\n\t\t<td data-tooltip-text=\"Click to get help with this purchase\" class=\"wht_items wht_item_refunded\"\">\n\t\t\t\t\t\t\t\t\t\t\t\t<div style=\"clear: both\">\n\t\t\t\t\t\tChivalry 2\t\t\t\t\t\t\t\t\t\t\t</div>\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t</td>\n\t\t<td class=\"wht_type wht_refunded\">\n\t\t\t\t\t\t\t\t\t<div>Purchase</div>\n\t\t\t\t\t\t<div class=\"wth_payment\">\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\tAliPay\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t</div>\n\t\t\t\t</td>\n\t\t<td class=\"wht_base_price\">\n\t\t\t¥ 116.00\t\t</td>\n\t\t<td class=\"wht_tax\">\n\t\t\t\t\t</td>\n\t\t<td class=\"wht_shipping\">\n\t\t\t\t\t</td>\n\t\t<td class=\"wht_total wht_refunded\">\n\t\t\t\t\t\t\t¥ 116.00\t\t\t\t\t</td>\n\n\t\t\t\t\t<td class=\"wht_wallet_change\"></td>\n\t\t\t<td class=\"wht_wallet_balance\"></td>\n\t\t\t</tr>\n\t\t\t<tr data-panel=\"{&quot;focusable&quot;:true,&quot;clickOnActivate&quot;:true}\" role=\"button\" class=\"wallet_table_row \" onclick=\"location.href='https://help.steampowered.com/en/wizard/HelpWithTransaction?transid=1478797762232260746'\">\n\t\t<td class=\"wht_date\">6 Nov, 2019</td>\n\t\t<td data-tooltip-text=\"Click to get help with this purchase\" class=\"wht_items \"\">\n\t\t\t\t\t\t\t\t\t\t\t\t<div style=\"clear: both\">\n\t\t\t\t\t\tWallpaper Engine\t\t\t\t\t\t\t\t\t\t\t</div>\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t</td>\n\t\t<td class=\"wht_type \">\n\t\t\t\t\t\t\t\t\t<div>Purchase</div>\n\t\t\t\t\t\t<div class=\"wth_payment\">\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\tAliPay\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t</div>\n\t\t\t\t</td>\n\t\t<td class=\"wht_base_price\">\n\t\t\t¥ 19.00\t\t</td>\n\t\t<td class=\"wht_tax\">\n\t\t\t\t\t</td>\n\t\t<td class=\"wht_shipping\">\n\t\t\t\t\t</td>\n\t\t<td class=\"wht_total \">\n\t\t\t\t\t\t\t¥ 19.00\t\t\t\t\t</td>\n\n\t\t\t\t\t<td class=\"wht_wallet_change\"></td>\n\t\t\t<td class=\"wht_wallet_balance\"></td>\n\t\t\t</tr>\n\t\t<tr data-panel=\"{&quot;focusable&quot;:true,&quot;clickOnActivate&quot;:true}\" role=\"button\" class=\"wallet_table_row \" onclick=\"location.href='https://help.steampowered.com/en/wizard/HelpWithTransaction?transid=1175846430789332971'\">\n\t\t<td class=\"wht_date\">24 Apr, 2017</td>\n\t\t<td data-tooltip-text=\"Click to get help with this purchase\" class=\"wht_items \"\">\n\t\t\t\t\t\t\t\t\t\t\t\t<div style=\"clear: both\">\n\t\t\t\t\t\tPLAYERUNKNOWN'S BATTLEGROUNDS - Worldwide Package\t\t\t\t\t\t\t\t\t\t\t</div>\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t</td>\n\t\t<td class=\"wht_type \">\n\t\t\t\t\t\t\t\t\t<div>Purchase</div>\n\t\t\t\t\t\t<div class=\"wth_payment\">\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\tAliPay\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t</div>\n\t\t\t\t</td>\n\t\t<td class=\"wht_base_price\">\n\t\t\t¥ 98.00\t\t</td>\n\t\t<td class=\"wht_tax\">\n\t\t\t\t\t</td>\n\t\t<td class=\"wht_shipping\">\n\t\t\t\t\t</td>\n\t\t<td class=\"wht_total \">\n\t\t\t\t\t\t\t¥ 98.00\t\t\t\t\t</td>\n\n\t\t\t\t\t<td class=\"wht_wallet_change\"></td>\n\t\t\t<td class=\"wht_wallet_balance\"></td>\n\t\t\t</tr>\n\t\t\t\t<tr id=\"more_history\" style=\"display: none\"></tr>\n\t\t\t</tbody>\n\t\t</table>\n\n\t\t<div class=\"load_more_history_area\">\n\t\t\t<div data-panel=\"{&quot;focusable&quot;:true,&quot;clickOnActivate&quot;:true}\" role=\"button\" id=\"load_more_button\" class=\"btnv6_blue_hoverfade btn_medium\" onclick=\"WalletHistory_LoadMore(); return false;\" style=\"display:none\" >\n\t\t\t\tLoad More Transactions\t\t\t</div>\n\t\t\t<div id=\"wallet_history_loading\" style=\"display: none; \">\n\t\t\t\t<img src=\"https://store.fastly.steamstatic.com/public/images/login/throbber.gif\">\n\t\t\t</div>\n\t\t</div>\n\t</div>"
      ]
    }
  ]
}
```



### 4. Amazon

- in progress


## Technical Data Structure

All attestation inputs sent to the zkVM follow the same top-level JSON shape. A concrete example is:

- [`zktls/data/github.json`](./zktls/data/github.json)

At the top level, the payload contains:

- `public_data`: The attestation generated by zkTLS, including the hash of the plaintext response returned by the data source.
- `private_data`: The plaintext response returned by the corresponding data source, used by the zkVM for business logic computation. For the business meaning of each source-specific field, refer to the [Data Sources and Format](#data-sources-and-format) section.

The zkVM logic in [`zktls/app/src/main.rs`](./zktls/app/src/main.rs) works as follows:

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
  --input ./zktls/data/github.json \
  --output-dir pico_out
```
