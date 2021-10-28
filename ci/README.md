Setup environment and run deploy script:  
```bash
$> export BUILD_SOURCE=/path/to/repo/deip-substrate 
$> export BUILD_CACHE=/path/to/build/cache/any/empty/directory
$> cd /path/to/repo/deip-substrate/ci
$> ./deploy.sh
```

Make sure everything good:  
```bash
$> docker-compose logs --follow deip-event-proxy-client
```
You should see something like this:
```text
deip-event-proxy-client_1  | Ok(Message { ptr: 0x7f351c0027a8 })
deip-event-proxy-client_1  | key: Some("events") ; topic: "blockchain" ; offset 132
deip-event-proxy-client_1  | {
deip-event-proxy-client_1  |   "type": "infrastructure",
deip-event-proxy-client_1  |   "name": "block_created",
deip-event-proxy-client_1  |   "data": {
deip-event-proxy-client_1  |     "number": 1,
deip-event-proxy-client_1  |     "hash": "0x5e8ab72539984edf84f8b65ee3911a52915f53b36ef34a7c3d90b8af27a061a4",
deip-event-proxy-client_1  |     "parent_hash": "0xab696ce2e32c00e071fb0f2154124596114a08142ee25cb0b1e5e331195b567c"
deip-event-proxy-client_1  |   },
deip-event-proxy-client_1  |   "meta": {
deip-event-proxy-client_1  |     "domain_events": 0
deip-event-proxy-client_1  |   }
deip-event-proxy-client_1  | }
...
```
