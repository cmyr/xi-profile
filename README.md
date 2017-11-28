# Very very basic xi profiling

This repo contains a simple harness for timing RPC performance.

It simulates a client, with no runloop; a series of simple init RPCs
are sent to core, and all received messages are logged and timestamped.

Because there is no runloop on the client side, this should give a
fairly accurate picture of core responsiveness in the most simple case.


## Use:

To setup: `make setup`

To run: `cargo run`

Results are printed to stdout:

```
2017-11-28 23:56:09.412369 UTC
butter.local Darwin v16.7.0 8x4009Mhz

###timestamped init###

11us       ->  {"method":"client_started","params":{"client_extras_dir":"plugins"}}
73us       ->  {"id":0,"method":"new_view","params":{"file_path":"./testdata/main.rs"}}
73us       ->  {"method":"edit","params":{"view_id":"view-id-1","method":"scroll","params":[0,1...
74us       ->  {"method":"edit","params":{"view_id":"view-id-1","method":"request_lines","param...
430us      <-  {"method":"available_themes","params":{"themes":["InspiredGitHub","Solarized (da...
556us      <-  {"id":0,"result":"view-id-1"}
937us      <-  {"method":"update","params":{"update":{"ops":[{"lines":[{"cursor":[0],"styles":[...
1.11ms     <-  {"method":"update","params":{"update":{"ops":[{"lines":[{"cursor":[0],"styles":[...
1.12ms     <-  {"method":"scroll_to","params":{"col":0,"line":0,"view_id":"view-id-1"}}
1.13ms     <-  {"method":"update","params":{"update":{"ops":[{"lines":[{"cursor":[0],"styles":[...
1.14ms     <-  {"method":"available_plugins","params":{"plugins":[{"name":"syntect","running":f...
1.15ms     <-  {"method":"config_changed","params":{"changes":{"font_face":"InconsolataGo","fon...
2.22ms     <-  {"method":"plugin_started","params":{"plugin":"syntect","view_id":"view-id-1"}}
2.22ms     <-  {"method":"update_cmds","params":{"cmds":[],"plugin":"syntect","view_id":"view-i...
30.304ms   <-  {"method":"def_style","params":{"fg_color":4289142109,"id":2,"weight":700}}
30.304ms   <-  {"method":"def_style","params":{"fg_color":4281479730,"id":3}}
30.304ms   <-  {"method":"def_style","params":{"fg_color":4286143907,"id":4,"weight":700}}
30.304ms   <-  {"method":"def_style","params":{"fg_color":4279776913,"id":5}}
30.304ms   <-  {"method":"def_style","params":{"fg_color":4284654428,"id":6}}
30.304ms   <-  {"method":"def_style","params":{"fg_color":4278224563,"id":7}}
30.308ms   <-  {"method":"update","params":{"update":{"ops":[{"lines":[{"cursor":[0],"styles":[...
34.340ms   <-  {"method":"def_style","params":{"fg_color":4281479730,"id":8}}
34.344ms   <-  {"method":"update","params":{"update":{"ops":[{"lines":[{"cursor":[0],"styles":[...
35.357ms   <-  {"method":"update","params":{"update":{"ops":[{"lines":[{"cursor":[0],"styles":[...
36.362ms   <-  {"method":"update","params":{"update":{"ops":[{"lines":[{"cursor":[0],"styles":[...

###sync roundtrip###

ran 100 sync RPC requests:
mean: 189us
min: 47us
max: 311us
```
