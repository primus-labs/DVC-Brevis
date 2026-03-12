## DVC-Brevis (ZKTLS Integration)

This example demonstrates how to verify diverse Web2 data sources by leveraging the synergy between **Primus zkTLS** and **Brevis zkVM**.

The end-to-end workflow follows a two-stage process:

1. **Data Attestation (Primus zkTLS):** On the client side, a browser extension uses zkTLS to generate cryptographic proofs of specific Web2 data items, ensuring data authenticity and privacy at the source.
2. **Computation & Verification (Brevis zkVM):** These raw data items, along with the hashed version, are then fed into the Brevis zkVM, which executes custom business logic (such as threshold checks, aggregations, or complex calculations) to generate a final verification. 

## Data Sources and Format

### 1. CEXs (Binance, OKX)

#### Binance

- **Description**: Verifies a user's Binance user ID, KYC level, and past 6 months spot trade history list.

- **zkTLS Verified Fields (`data` object)**:
  - `userId`: The unique identifier for the Binance account.
  - `passKycLevel`: The current KYC level of the user (e.g., `"INTERMEDIATE"`).
  - `list`: A verified list of spot transaction history in the past 6 months.
 
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
        "[{\"tradeId\":327619867,\"orderId\":8131702704,\"tradeIdStr\":\"327619867\",\"price\":\"64000\",\"time\":1770349700000,\"symbol\":\"BTCUSDC\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.00156\",\"fee\":\"0.00012225\",\"feeAsset\":\"BNB\",\"totalQuota\":\"99.84\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDC\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":99.84,\"usdtRate\":64000.00000000},{\"tradeId\":326778255,\"orderId\":8034497893,\"tradeIdStr\":\"326778255\",\"price\":\"66000\",\"time\":1770315414000,\"symbol\":\"BTCUSDC\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.00151\",\"fee\":\"0.00011463\",\"feeAsset\":\"BNB\",\"totalQuota\":\"99.66\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDC\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":99.66,\"usdtRate\":66000.00000000},{\"tradeId\":326549251,\"orderId\":8034492826,\"tradeIdStr\":\"326549251\",\"price\":\"68000\",\"time\":1770304376000,\"symbol\":\"BTCUSDC\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.00147\",\"fee\":\"0.00011331\",\"feeAsset\":\"BNB\",\"totalQuota\":\"99.96\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDC\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":99.96,\"usdtRate\":68000.00000000},{\"tradeId\":326353752,\"orderId\":8034469879,\"tradeIdStr\":\"326353752\",\"price\":\"70000\",\"time\":1770290677000,\"symbol\":\"BTCUSDC\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.00142\",\"fee\":\"0.00010992\",\"feeAsset\":\"BNB\",\"totalQuota\":\"99.4\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDC\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":99.40,\"usdtRate\":70000.00000000},{\"tradeId\":325804454,\"orderId\":8034466609,\"tradeIdStr\":\"325804454\",\"price\":\"72000\",\"time\":1770242749000,\"symbol\":\"BTCUSDC\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.00138\",\"fee\":\"0.00010725\",\"feeAsset\":\"BNB\",\"totalQuota\":\"99.36\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDC\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":99.36,\"usdtRate\":72000.00000000},{\"tradeId\":324461619,\"orderId\":8034458855,\"tradeIdStr\":\"324461619\",\"price\":\"74000\",\"time\":1770143134000,\"symbol\":\"BTCUSDC\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.00135\",\"fee\":\"0.00010132\",\"feeAsset\":\"BNB\",\"totalQuota\":\"99.9\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDC\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":99.90,\"usdtRate\":74000.00000000},{\"tradeId\":321200621,\"orderId\":7259447524,\"tradeIdStr\":\"321200621\",\"price\":\"80000\",\"time\":1769878426000,\"symbol\":\"BTCUSDC\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.0025\",\"fee\":\"0.00018914\",\"feeAsset\":\"BNB\",\"totalQuota\":\"200\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDC\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":200.00,\"usdtRate\":80000.00000000},{\"tradeId\":323961057,\"orderId\":8070660152,\"tradeIdStr\":\"323961057\",\"price\":\"2500\",\"time\":1769878062000,\"symbol\":\"ETHUSDC\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.04\",\"fee\":\"0.00009439\",\"feeAsset\":\"BNB\",\"totalQuota\":\"100\",\"productName\":\"\",\"baseAsset\":\"ETH\",\"quoteAsset\":\"USDC\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":100.00,\"usdtRate\":2500.00000000},{\"tradeId\":323669739,\"orderId\":8070658929,\"tradeIdStr\":\"323669739\",\"price\":\"2600\",\"time\":1769868878000,\"symbol\":\"ETHUSDC\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.0384\",\"fee\":\"0.00009108\",\"feeAsset\":\"BNB\",\"totalQuota\":\"99.84\",\"productName\":\"\",\"baseAsset\":\"ETH\",\"quoteAsset\":\"USDC\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":99.84,\"usdtRate\":2600.00000000},{\"tradeId\":286736355,\"orderId\":8072548034,\"tradeIdStr\":\"286736355\",\"price\":\"2700\",\"time\":1763710427000,\"symbol\":\"ETHUSDC\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.037\",\"fee\":\"0.0000896\",\"feeAsset\":\"BNB\",\"totalQuota\":\"99.9\",\"productName\":\"\",\"baseAsset\":\"ETH\",\"quoteAsset\":\"USDC\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":99.90,\"usdtRate\":2700.00000000},{\"tradeId\":279494954,\"orderId\":7259440637,\"tradeIdStr\":\"279494954\",\"price\":\"85000\",\"time\":1763710368000,\"symbol\":\"BTCUSDC\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.00117\",\"fee\":\"0.00008837\",\"feeAsset\":\"BNB\",\"totalQuota\":\"99.45\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDC\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":99.45,\"usdtRate\":85000.00000000},{\"tradeId\":279271058,\"orderId\":7206158659,\"tradeIdStr\":\"279271058\",\"price\":\"86000\",\"time\":1763693080000,\"symbol\":\"BTCUSDC\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.00232\",\"fee\":\"0.0001733\",\"feeAsset\":\"BNB\",\"totalQuota\":\"199.52\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDC\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":199.52,\"usdtRate\":86000.00000000},{\"tradeId\":286249957,\"orderId\":8005554007,\"tradeIdStr\":\"286249957\",\"price\":\"2800\",\"time\":1763665343000,\"symbol\":\"ETHUSDC\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.0535\",\"fee\":\"0.00012955\",\"feeAsset\":\"BNB\",\"totalQuota\":\"149.8\",\"productName\":\"\",\"baseAsset\":\"ETH\",\"quoteAsset\":\"USDC\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":149.80,\"usdtRate\":2800.00000000},{\"tradeId\":276491165,\"orderId\":7188241487,\"tradeIdStr\":\"276491165\",\"price\":\"92000\",\"time\":1763407970000,\"symbol\":\"BTCUSDC\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.00108\",\"fee\":\"0.00008267\",\"feeAsset\":\"BNB\",\"totalQuota\":\"99.36\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDC\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":99.36,\"usdtRate\":92000.00000000},{\"tradeId\":283757382,\"orderId\":7983217024,\"tradeIdStr\":\"283757382\",\"price\":\"3000\",\"time\":1763407967000,\"symbol\":\"ETHUSDC\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.0333\",\"fee\":\"0.00008319\",\"feeAsset\":\"BNB\",\"totalQuota\":\"99.9\",\"productName\":\"\",\"baseAsset\":\"ETH\",\"quoteAsset\":\"USDC\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":99.90,\"usdtRate\":3000.00000000},{\"tradeId\":283668343,\"orderId\":7983201965,\"tradeIdStr\":\"283668343\",\"price\":\"3050\",\"time\":1763402529000,\"symbol\":\"ETHUSDC\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.0075\",\"fee\":\"0.00001887\",\"feeAsset\":\"BNB\",\"totalQuota\":\"22.875\",\"productName\":\"\",\"baseAsset\":\"ETH\",\"quoteAsset\":\"USDC\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":22.87,\"usdtRate\":3050.00000000},{\"tradeId\":283668342,\"orderId\":7983201965,\"tradeIdStr\":\"283668342\",\"price\":\"3050\",\"time\":1763402529000,\"symbol\":\"ETHUSDC\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.0088\",\"fee\":\"0.00002214\",\"feeAsset\":\"BNB\",\"totalQuota\":\"26.84\",\"productName\":\"\",\"baseAsset\":\"ETH\",\"quoteAsset\":\"USDC\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":26.84,\"usdtRate\":3050.00000000},{\"tradeId\":276360663,\"orderId\":7188240237,\"tradeIdStr\":\"276360663\",\"price\":\"93000\",\"time\":1763396416000,\"symbol\":\"BTCUSDC\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.00053\",\"fee\":\"0.00004102\",\"feeAsset\":\"BNB\",\"totalQuota\":\"49.29\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDC\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":49.29,\"usdtRate\":93000.00000000},{\"tradeId\":283400358,\"orderId\":7983195594,\"tradeIdStr\":\"283400358\",\"price\":\"3100\",\"time\":1763387528000,\"symbol\":\"ETHUSDC\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.0161\",\"fee\":\"0.0000412\",\"feeAsset\":\"BNB\",\"totalQuota\":\"49.91\",\"productName\":\"\",\"baseAsset\":\"ETH\",\"quoteAsset\":\"USDC\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":49.91,\"usdtRate\":3100.00000000},{\"tradeId\":276230877,\"orderId\":7188238784,\"tradeIdStr\":\"276230877\",\"price\":\"94000\",\"time\":1763386862000,\"symbol\":\"BTCUSDC\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.00053\",\"fee\":\"0.00004074\",\"feeAsset\":\"BNB\",\"totalQuota\":\"49.82\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDC\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":49.82,\"usdtRate\":94000.00000000},{\"tradeId\":3166421928,\"orderId\":38275828389,\"tradeIdStr\":\"3166421928\",\"price\":\"3100\",\"time\":1763123663000,\"symbol\":\"ETHUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.0322\",\"fee\":\"0.00008355\",\"feeAsset\":\"BNB\",\"totalQuota\":\"99.82\",\"productName\":\"\",\"baseAsset\":\"ETH\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":99.82,\"usdtRate\":3100.00000000},{\"tradeId\":1307459188,\"orderId\":10100247507,\"tradeIdStr\":\"1307459188\",\"price\":\"900\",\"time\":1763122951000,\"symbol\":\"BNBUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.011\",\"fee\":\"0.00000825\",\"feeAsset\":\"BNB\",\"totalQuota\":\"9.9\",\"productName\":\"\",\"baseAsset\":\"BNB\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":9.90,\"usdtRate\":900.00000000},{\"tradeId\":1715347685,\"orderId\":15302425769,\"tradeIdStr\":\"1715347685\",\"price\":\"140\",\"time\":1763120614000,\"symbol\":\"SOLUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.071\",\"fee\":\"0.00000822\",\"feeAsset\":\"BNB\",\"totalQuota\":\"9.94\",\"productName\":\"\",\"baseAsset\":\"SOL\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":9.94,\"usdtRate\":140.00000000},{\"tradeId\":5488936960,\"orderId\":51960893108,\"tradeIdStr\":\"5488936960\",\"price\":\"96000\",\"time\":1763120571000,\"symbol\":\"BTCUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.00104\",\"fee\":\"0.00008268\",\"feeAsset\":\"BNB\",\"totalQuota\":\"99.84\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":99.84,\"usdtRate\":96000.00000000},{\"tradeId\":3165879626,\"orderId\":38883955356,\"tradeIdStr\":\"3165879626\",\"price\":\"3150\",\"time\":1763119709000,\"symbol\":\"ETHUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.0158\",\"fee\":\"0.000041\",\"feeAsset\":\"BNB\",\"totalQuota\":\"49.77\",\"productName\":\"\",\"baseAsset\":\"ETH\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":49.77,\"usdtRate\":3150.00000000},{\"tradeId\":3164647469,\"orderId\":38883976157,\"tradeIdStr\":\"3164647469\",\"price\":\"3200\",\"time\":1763105844000,\"symbol\":\"ETHUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.0156\",\"fee\":\"0.00004069\",\"feeAsset\":\"BNB\",\"totalQuota\":\"49.92\",\"productName\":\"\",\"baseAsset\":\"ETH\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":49.92,\"usdtRate\":3200.00000000},{\"tradeId\":5487742226,\"orderId\":51960901299,\"tradeIdStr\":\"5487742226\",\"price\":\"97000\",\"time\":1763105824000,\"symbol\":\"BTCUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.00103\",\"fee\":\"0.0000814\",\"feeAsset\":\"BNB\",\"totalQuota\":\"99.91\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":99.91,\"usdtRate\":97000.00000000},{\"tradeId\":1714055347,\"orderId\":15167511436,\"tradeIdStr\":\"1714055347\",\"price\":\"140\",\"time\":1763095109000,\"symbol\":\"SOLUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.071\",\"fee\":\"0.00000828\",\"feeAsset\":\"BNB\",\"totalQuota\":\"9.94\",\"productName\":\"\",\"baseAsset\":\"SOL\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":9.94,\"usdtRate\":140.00000000},{\"tradeId\":5481484878,\"orderId\":51853603118,\"tradeIdStr\":\"5481484878\",\"price\":\"100000\",\"time\":1763055682000,\"symbol\":\"BTCUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.002\",\"fee\":\"0.00016018\",\"feeAsset\":\"BNB\",\"totalQuota\":\"200\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":200.00,\"usdtRate\":100000.00000000},{\"tradeId\":5450389549,\"orderId\":51359251040,\"tradeIdStr\":\"5450389549\",\"price\":\"100000\",\"time\":1762514127000,\"symbol\":\"BTCUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.002\",\"fee\":\"0.00015938\",\"feeAsset\":\"BNB\",\"totalQuota\":\"200\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":200.00,\"usdtRate\":100000.00000000},{\"tradeId\":49526707,\"orderId\":629186063,\"tradeIdStr\":\"49526707\",\"price\":\"0.05\",\"time\":1762443390000,\"symbol\":\"PHAUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"100\",\"fee\":\"0.00000402\",\"feeAsset\":\"BNB\",\"totalQuota\":\"5\",\"productName\":\"\",\"baseAsset\":\"PHA\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":5.00,\"usdtRate\":0.05000000},{\"tradeId\":5434466874,\"orderId\":51278011458,\"tradeIdStr\":\"5434466874\",\"price\":\"100000\",\"time\":1762287472000,\"symbol\":\"BTCUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.001\",\"fee\":\"0.00008317\",\"feeAsset\":\"BNB\",\"totalQuota\":\"100\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":100.00,\"usdtRate\":100000.00000000},{\"tradeId\":3103350701,\"orderId\":38192135134,\"tradeIdStr\":\"3103350701\",\"price\":\"3400\",\"time\":1762276075000,\"symbol\":\"ETHUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.0588\",\"fee\":\"0.00016284\",\"feeAsset\":\"BNB\",\"totalQuota\":\"199.92\",\"productName\":\"\",\"baseAsset\":\"ETH\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":199.92,\"usdtRate\":3400.00000000},{\"tradeId\":5431275610,\"orderId\":51287341287,\"tradeIdStr\":\"5431275610\",\"price\":\"103000\",\"time\":1762267068000,\"symbol\":\"BTCUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.00097\",\"fee\":\"0.00007963\",\"feeAsset\":\"BNB\",\"totalQuota\":\"99.91\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":99.91,\"usdtRate\":103000.00000000},{\"tradeId\":5429717573,\"orderId\":51285805766,\"tradeIdStr\":\"5429717573\",\"price\":\"104000\",\"time\":1762246364000,\"symbol\":\"BTCUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.00096\",\"fee\":\"0.000079\",\"feeAsset\":\"BNB\",\"totalQuota\":\"99.84\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":99.84,\"usdtRate\":104000.00000000},{\"tradeId\":3099590316,\"orderId\":38181977349,\"tradeIdStr\":\"3099590316\",\"price\":\"3500\",\"time\":1762239800000,\"symbol\":\"ETHUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.0571\",\"fee\":\"0.00015732\",\"feeAsset\":\"BNB\",\"totalQuota\":\"199.85\",\"productName\":\"\",\"baseAsset\":\"ETH\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":199.85,\"usdtRate\":3500.00000000},{\"tradeId\":5428568620,\"orderId\":51263870254,\"tradeIdStr\":\"5428568620\",\"price\":\"105306\",\"time\":1762234764000,\"symbol\":\"BTCUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.00094\",\"fee\":\"0.00007722\",\"feeAsset\":\"BNB\",\"totalQuota\":\"98.98764\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":98.99,\"usdtRate\":105306.00000000},{\"tradeId\":1683817886,\"orderId\":15076527158,\"tradeIdStr\":\"1683817886\",\"price\":\"163\",\"time\":1762233550000,\"symbol\":\"SOLUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.061\",\"fee\":\"0.00000761\",\"feeAsset\":\"BNB\",\"totalQuota\":\"9.943\",\"productName\":\"\",\"baseAsset\":\"SOL\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":9.94,\"usdtRate\":163.00000000},{\"tradeId\":3097975850,\"orderId\":38167202195,\"tradeIdStr\":\"3097975850\",\"price\":\"3620\",\"time\":1762224515000,\"symbol\":\"ETHUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.0276\",\"fee\":\"0.00007554\",\"feeAsset\":\"BNB\",\"totalQuota\":\"99.912\",\"productName\":\"\",\"baseAsset\":\"ETH\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":99.91,\"usdtRate\":3620.00000000},{\"tradeId\":1681328151,\"orderId\":15053131649,\"tradeIdStr\":\"1681328151\",\"price\":\"170\",\"time\":1762183651000,\"symbol\":\"SOLUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.058\",\"fee\":\"0.00000745\",\"feeAsset\":\"BNB\",\"totalQuota\":\"9.86\",\"productName\":\"\",\"baseAsset\":\"SOL\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":9.86,\"usdtRate\":170.00000000},{\"tradeId\":5422575765,\"orderId\":51214560976,\"tradeIdStr\":\"5422575765\",\"price\":\"107000\",\"time\":1762167552000,\"symbol\":\"BTCUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.00046\",\"fee\":\"0.00003648\",\"feeAsset\":\"BNB\",\"totalQuota\":\"49.22\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":49.22,\"usdtRate\":107000.00000000},{\"tradeId\":3091069429,\"orderId\":38098470715,\"tradeIdStr\":\"3091069429\",\"price\":\"3720\",\"time\":1762142814000,\"symbol\":\"ETHUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.0268\",\"fee\":\"0.00007201\",\"feeAsset\":\"BNB\",\"totalQuota\":\"99.696\",\"productName\":\"\",\"baseAsset\":\"ETH\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":99.70,\"usdtRate\":3720.00000000},{\"tradeId\":3090320128,\"orderId\":38091703465,\"tradeIdStr\":\"3090320128\",\"price\":\"3820\",\"time\":1762136853000,\"symbol\":\"ETHUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.013\",\"fee\":\"0.00003499\",\"feeAsset\":\"BNB\",\"totalQuota\":\"49.66\",\"productName\":\"\",\"baseAsset\":\"ETH\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":49.66,\"usdtRate\":3820.00000000},{\"tradeId\":3045006324,\"orderId\":37642430975,\"tradeIdStr\":\"3045006324\",\"price\":\"3820\",\"time\":1761188129000,\"symbol\":\"ETHUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.013\",\"fee\":\"0.00003437\",\"feeAsset\":\"BNB\",\"totalQuota\":\"49.66\",\"productName\":\"\",\"baseAsset\":\"ETH\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":49.66,\"usdtRate\":3820.00000000},{\"tradeId\":1656565260,\"orderId\":14840956546,\"tradeIdStr\":\"1656565260\",\"price\":\"180\",\"time\":1761162743000,\"symbol\":\"SOLUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.055\",\"fee\":\"0.00000693\",\"feeAsset\":\"BNB\",\"totalQuota\":\"9.9\",\"productName\":\"\",\"baseAsset\":\"SOL\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":9.90,\"usdtRate\":180.00000000},{\"tradeId\":3011422740,\"orderId\":37332737841,\"tradeIdStr\":\"3011422740\",\"price\":\"3700\",\"time\":1760691767000,\"symbol\":\"ETHUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.027\",\"fee\":\"0.00007182\",\"feeAsset\":\"BNB\",\"totalQuota\":\"99.9\",\"productName\":\"\",\"baseAsset\":\"ETH\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":99.90,\"usdtRate\":3700.00000000},{\"tradeId\":3009846578,\"orderId\":37084625092,\"tradeIdStr\":\"3009846578\",\"price\":\"3800\",\"time\":1760684800000,\"symbol\":\"ETHUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.0263\",\"fee\":\"0.00006889\",\"feeAsset\":\"BNB\",\"totalQuota\":\"99.94\",\"productName\":\"\",\"baseAsset\":\"ETH\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":99.94,\"usdtRate\":3800.00000000},{\"tradeId\":1641106756,\"orderId\":14756202245,\"tradeIdStr\":\"1641106756\",\"price\":\"180\",\"time\":1760683796000,\"symbol\":\"SOLUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.055\",\"fee\":\"0.00000675\",\"feeAsset\":\"BNB\",\"totalQuota\":\"9.9\",\"productName\":\"\",\"baseAsset\":\"SOL\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":9.90,\"usdtRate\":180.00000000},{\"tradeId\":3009166165,\"orderId\":37307700866,\"tradeIdStr\":\"3009166165\",\"price\":\"3900\",\"time\":1760679786000,\"symbol\":\"ETHUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.0128\",\"fee\":\"0.00003295\",\"feeAsset\":\"BNB\",\"totalQuota\":\"49.92\",\"productName\":\"\",\"baseAsset\":\"ETH\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":49.92,\"usdtRate\":3900.00000000},{\"tradeId\":3000895948,\"orderId\":37237509096,\"tradeIdStr\":\"3000895948\",\"price\":\"3999.34\",\"time\":1760595017000,\"symbol\":\"ETHUSDT\",\"side\":\"BUY\",\"activeBuy\":true,\"realPnl\":0,\"qty\":\"0.0025\",\"fee\":\"0.00000633\",\"feeAsset\":\"BNB\",\"totalQuota\":\"9.99835\",\"productName\":\"\",\"baseAsset\":\"ETH\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"TAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":10.00,\"usdtRate\":3999.34000000},{\"tradeId\":5347098692,\"orderId\":50269713878,\"tradeIdStr\":\"5347098692\",\"price\":\"110976.93\",\"time\":1760594973000,\"symbol\":\"BTCUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.00009\",\"fee\":\"0.00000633\",\"feeAsset\":\"BNB\",\"totalQuota\":\"9.9879237\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":9.99,\"usdtRate\":110976.93000000},{\"tradeId\":5346658828,\"orderId\":50263643820,\"tradeIdStr\":\"5346658828\",\"price\":\"111485.42\",\"time\":1760583009000,\"symbol\":\"BTCUSDT\",\"side\":\"BUY\",\"activeBuy\":true,\"realPnl\":0,\"qty\":\"0.00008\",\"fee\":\"0.00000561\",\"feeAsset\":\"BNB\",\"totalQuota\":\"8.9188336\",\"productName\":\"\",\"baseAsset\":\"BTC\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"TAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":8.92,\"usdtRate\":111485.42000000},{\"tradeId\":2985637479,\"orderId\":37084609858,\"tradeIdStr\":\"2985637479\",\"price\":\"3900\",\"time\":1760437917000,\"symbol\":\"ETHUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.0128\",\"fee\":\"0.00003179\",\"feeAsset\":\"BNB\",\"totalQuota\":\"49.92\",\"productName\":\"\",\"baseAsset\":\"ETH\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":49.92,\"usdtRate\":3900.00000000},{\"tradeId\":1228969253,\"orderId\":9468281225,\"tradeIdStr\":\"1228969253\",\"price\":\"1197.84\",\"time\":1760423768000,\"symbol\":\"BNBUSDT\",\"side\":\"BUY\",\"activeBuy\":false,\"realPnl\":0,\"qty\":\"0.008\",\"fee\":\"0.000006\",\"feeAsset\":\"BNB\",\"totalQuota\":\"9.58272\",\"productName\":\"\",\"baseAsset\":\"BNB\",\"quoteAsset\":\"USDT\",\"money\":null,\"userId\":782151446,\"userIdStr\":\"782151446\",\"email\":null,\"role\":\"MAKER\",\"busdAmount\":null,\"busdRate\":null,\"isSor\":false,\"usdtAmount\":9.58,\"usdtRate\":1197.84000000}]"
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
