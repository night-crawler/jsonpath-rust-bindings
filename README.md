# jsonpath-rust-bindings

Python bindings for [jsonpath-rust](https://github.com/besok/jsonpath-rust).

## Installation

```bash
pip install jsonpath-rust-bindings
```

## Usage

```python
from jsonpath_rust_bindings import Finder

sample = {
    "store": {
        "book": [
            {
                "category": "reference",
                "author": "Nigel Rees",
                "title": "Sayings of the Century",
                "price": 8.95,
            },
            {
                "category": "fiction",
                "author": "Evelyn Waugh",
                "title": "Sword of Honour",
                "price": 12.99,
            },
            {
                "category": "fiction",
                "author": "Herman Melville",
                "title": "Moby Dick",
                "isbn": "0-553-21311-3",
                "price": 8.99,
            },
            {
                "category": "fiction",
                "author": "J. R. R. Tolkien",
                "title": "The Lord of the Rings",
                "isbn": "0-395-19395-8",
                "price": 22.99,
            },
        ],
        "bicycle": {"color": "red", "price": 19.95},
    },
    "expensive": 10,
}

queries = [
    '$.store.book[*].author',
    '$..book[?(@.isbn)]',
    '$.store.*',
    '$..author',
    '$.store..price',
    '$..book[2]',
    # '$..book[-2]',
    '$..book[0,1]',
    '$..book[:2]',
    '$..book[1:2]',
    '$..book[-2:]',
    '$..book[2:]',
    '$.store.book[?(@.price<10)]',
    '$..book[?(@.price<=$.expensive)]',
    "$..book[?(@.author ~= '.*Rees')].price",
    '$..*',
]

f = Finder(sample)

for query in queries:
    print(query, f.find(query), '\n')

```

## Caveats

THe current implementation cloning the original PyObject when initializing the `Finder` instance. 
It has yet another consequence demonstrated in the following example:

```python
Python 3.11.5 (main, Aug 30 2023, 19:09:52) [GCC 13.2.1 20230730] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> original_object_i_want_to_mutate = {'a': {'b': 'sample b'}}
>>> from jsonpath_rust_bindings import Finder
>>> f = Finder(original_object_i_want_to_mutate)
>>> b_dict = f.find('$.a')[0].data
>>> b_dict
{'b': 'sample b'}
>>> b_dict['new'] = 42
>>> original_object_i_want_to_mutate
{'a': {'b': 'sample b'}}
```
