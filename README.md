Wrapper around [ttf_word_wrap](https://sr.ht/~halzy/ttf_word_wrap/) and [owned_ttf_parser](https://github.com/alexheretic/owned-ttf-parser) for use in Python.

### Usage
```python
# ./.env/bin/ipython
In [1]: import pyttfwrap

In [2]: wrapper = pyttfwrap.TextWrapper('/usr/share/fonts/TTF/iosevka-regular.ttf', '0')

In [3]: wrapper.wrap(42, 'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi utaliquip ex ea commodo consequat.')

Out[3]:
['Lorem ipsum dolor sit amet, consectetur',
 'adipiscing elit, sed do eiusmod tempor',
 'incididunt ut labore et dolore magna',
 'aliqua. Ut enim ad minim veniam, quis',
 'nostrud exercitation ullamco laboris nisi',
 'ut aliquip ex ea commodo consequat.']

```


### Building
```bash
$ python -m venv .venv
$ ./venv/bin/pip install maturin
$ ./venv/bin/maturin build --release
```
