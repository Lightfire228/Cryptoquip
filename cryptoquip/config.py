from pathlib import Path

import json

def _load_config():
    file_ = Path('./config.json')

    if not file_.exists():
        _load_default()

    try: 
        config_text = file_.read_text()
        return json.loads(config_text)
    except:
        return None

def _load_default():
    src  = Path('./config.default.json')
    dest = Path('./config.json')

    dest.write_text(src.read_text())

def _read_recurse(keys, config_):

    key, *rest = keys

    if len(rest) == 0:
        return config_[key]
    
    return _read_recurse(rest, config_[key])


def read_config(*keys, default=None):
    
    try:
        return _read_recurse(keys, config)
    except:
        print('Config invalid or unable to parse, using default value')
        return default


config = _load_config()