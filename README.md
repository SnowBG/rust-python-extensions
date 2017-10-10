compiles rust code into a python module

## steps to reproduce

```
$ cargo build
$ cp ./target/debug/libexample.so example.so
$ python3
```


```python
import example
print(example.double(7))
>>> 14
```