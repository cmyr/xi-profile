# Very very basic xi profiling

This repo contains a simple harness for timing RPC performance.

It simulates a client, with no runloop; a series of simple init RPCs
are sent to core, and all received messages are logged and timestamped.

Because there is no runloop on the client side, this should give a
fairly accurate picture of core responsiveness in the most simple case.


## Use:

To setup: `make setup`

To run: `cargo run --release`

Results are printed to stdout:

```
2017-11-29 00:37:47.165294 UTC
Darwin v16.7.0
cpu: 8 @ 4009MHz, load: 1.37/2.44/2.45
MemInfo { total: 33554432, free: 1886124, avail: 1026600, buffers: 0, cached: 0, swap_total: 0, swap_free: 0 }

###timestamped init###

3us        ->  {"method":"client_started","params":{"client_extras_dir":"plugins"}}
18us       ->  {"id":0,"method":"new_view","params":{"file_path":"./testdata/main.rs"}}
19us       ->  {"method":"edit","params":{"view_id":"view-id-1","method":"scroll","params":[0,1...
19us       ->  {"method":"edit","params":{"view_id":"view-id-1","method":"request_lines","param...
477us      <-  {"method":"available_themes","params":{"themes":["InspiredGitHub","Solarized (da...
631us      <-  {"id":0,"result":"view-id-1"}
1.0ms      <-  {"method":"update","params":{"update":{"ops":[{"lines":[{"cursor":[0],"styles":[...
1.3ms      <-  {"method":"update","params":{"update":{"ops":[{"lines":[{"cursor":[0],"styles":[...
1.3ms      <-  {"method":"scroll_to","params":{"col":0,"line":0,"view_id":"view-id-1"}}
1.5ms      <-  {"method":"update","params":{"update":{"ops":[{"lines":[{"cursor":[0],"styles":[...
1.6ms      <-  {"method":"available_plugins","params":{"plugins":[{"name":"syntect","running":f...
1.6ms      <-  {"method":"config_changed","params":{"changes":{"font_face":"InconsolataGo","fon...
2.8ms      <-  {"method":"plugin_started","params":{"plugin":"syntect","view_id":"view-id-1"}}
2.8ms      <-  {"method":"update_cmds","params":{"cmds":[],"plugin":"syntect","view_id":"view-i...
33.8ms     <-  {"method":"def_style","params":{"fg_color":4289142109,"id":2,"weight":700}}
33.8ms     <-  {"method":"def_style","params":{"fg_color":4281479730,"id":3}}
33.8ms     <-  {"method":"def_style","params":{"fg_color":4286143907,"id":4,"weight":700}}
33.8ms     <-  {"method":"def_style","params":{"fg_color":4279776913,"id":5}}
33.8ms     <-  {"method":"def_style","params":{"fg_color":4284654428,"id":6}}
33.8ms     <-  {"method":"def_style","params":{"fg_color":4278224563,"id":7}}
34.2ms     <-  {"method":"update","params":{"update":{"ops":[{"lines":[{"cursor":[0],"styles":[...
37.5ms     <-  {"method":"def_style","params":{"fg_color":4281479730,"id":8}}
37.9ms     <-  {"method":"update","params":{"update":{"ops":[{"lines":[{"cursor":[0],"styles":[...
39.2ms     <-  {"method":"update","params":{"update":{"ops":[{"lines":[{"cursor":[0],"styles":[...
39.7ms     <-  {"method":"update","params":{"update":{"ops":[{"lines":[{"cursor":[0],"styles":[...

###sync roundtrip###

ran 100 sync RPC requests:
mean: 92us
min: 37us
max: 594us
```
