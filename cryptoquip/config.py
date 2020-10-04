from pathlib import Path

import json

def _load_config():
    file_ = Path('./config.json')

    try: 
        config_text = file_.read_text()
    except:
        config_text = '{}'
    
    return json.loads(config_text)

config = _load_config()