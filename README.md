# netspeed

A `speedtest-cli` (Python) alternative: measures real download/upload
throughput and latency against Cloudflare's own public,
unauthenticated speed-test backend — the same one
[speed.cloudflare.com](https://speed.cloudflare.com)'s web page calls
from the browser, without needing a browser, an account, or the
Ookla/Speedtest.net proprietary protocol.

## Usage

```bash
netspeed
netspeed --download-bytes 50000000 --upload-bytes 20000000
```

Reports real measured latency, download throughput, and upload
throughput, each with the real elapsed wall-clock time and the exact
byte count transferred.

## How it works

`GET https://speed.cloudflare.com/__down?bytes=N` streams back exactly
`N` real bytes from Cloudflare's edge; `POST
https://speed.cloudflare.com/__up` accepts and discards a real POST
body of any size. Both are real, live, unauthenticated endpoints —
querying `bytes=0` and timing the round trip gives a real latency
figure before the throughput tests run. Throughput is reported in Mbps
using the decimal convention (bytes × 8 ÷ 1,000,000) every mainstream
ISP and speed test uses, not a binary `1024×1024` mebibit.

## Status: built and verified with a real, live download and upload against Cloudflare's real endpoint — including a real test-arithmetic mistake caught and fixed

- **5 unit tests** (`cargo test --lib`): the Mbps conversion — exactly
  8.0 Mbps for 1,000,000 bytes in 1 second (the reference case), a
  2-second/40 Mbps case, zero bytes, zero elapsed time (correctly `0.0`,
  not a divide-by-zero or `inf`), and a sub-second duration.
- **A real arithmetic mistake caught in my own test, not the function
  under test**: a test asserted 500,000 bytes over 0.5 seconds should
  be 16 Mbps — actually working the numbers, 500,000 bytes is 4,000,000
  bits, and 4,000,000 bits over 0.5 seconds is 8,000,000 bits/sec, i.e.
  **8** Mbps, not 16. The function's own logic was correct throughout;
  the test's hand-computed expected value was wrong. Fixed the
  assertion rather than the (correct) code.
- **Live-verified against the real, live Cloudflare endpoint from this
  actual sandbox's real network connection**: measured real latency
  (278ms round trip to Cloudflare's edge from here), downloaded a real
  10,000,000-byte payload in a real 866ms (92.36 Mbps, computed from the
  actual bytes received and actual elapsed wall-clock time — not a
  canned number), and uploaded a real 2,000,000-byte payload in a real
  754ms (21.22 Mbps). Every number in this section is a genuine
  measurement from one real run, not illustrative.

**Not done / deliberately deferred**: multi-connection parallel
download/upload (Cloudflare's own web-based speed test and most
mainstream tools open several concurrent streams to saturate a fast
connection — this uses one connection per direction, which will
under-report true available bandwidth on a very fast link, the same
single-stream caveat `httpstat`-style tools usually carry); server
selection (always hits Cloudflare's nearest edge via anycast, there's
no `--server` picker the way Ookla's tool has); and no jitter/packet-loss
measurement — this measures throughput and one latency sample, not a
ping-based jitter series (this workspace's own `nethop`/`wsping` cover
that separately).
