import json

from . import dirs

_config_data = None
config       = None

def _load_config():
    file_ = dirs.INSTALL_DIR / 'config.json'

    if not file_.exists():
        _load_default()

    try: 
        config_text = file_.read_text()
        return json.loads(config_text)
    except:
        return None

def _load_default():
    src  = dirs.APP_DIR     / 'config.default.json'
    dest = dirs.INSTALL_DIR / 'config.json'

    dest.write_text(src.read_text())

def _read_recurse(keys, config_):

    key, *rest = keys

    if len(rest) == 0:
        return config_[key]
    
    return _read_recurse(rest, config_[key])


def _read_config(*keys, default=None, relative_to=None):
    
    relative_to = relative_to or _config_data

    try:
        return _read_recurse(keys, relative_to)
    except:
        return default

class Config():

    def __init__(self, attrs=[]):
        self._attrs = attrs

    def __getattr__(self, key):
        return Config([*self._attrs, key])

    def resolve(self, default=None):
        return _read_config(*self._attrs, default=default)

_config_data = _load_config()
config       = Config()