# jsonpath-rust-bindings

![PyPI - Downloads](https://img.shields.io/pypi/dm/jsonpath-rust-bindings)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/night-crawler/jsonpath-rust-bindings/CI.yml)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/night-crawler/jsonpath-rust-bindings/test.yml?label=tests)
![piwheels (including prereleases)](https://img.shields.io/piwheels/v/jsonpath-rust-bindings)

This package contains Python bindings for [jsonpath-rust](https://github.com/besok/jsonpath-rust) library by [besok](https://github.com/besok).

The details regarding the JsonPath itself can be found [here](https://goessner.net/articles/JsonPath/).

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
    # print(query, f.find(query), '\n')
    print(query)
    print(f.find(query))
    print(f.find_data(query))
    print(f.find_absolute_path(query))

# You will see a bunch of found items like
# $.store.book[*].author
# [JsonPathResult(data='Nigel Rees', path="$['store']['book'][0]['author']"), JsonPathResult(data='Evelyn Waugh', path="$['store']['book'][1]['author']"), JsonPathResult(data='Herman Melville', path="$['store']['book'][2]['author']"), JsonPathResult(data='J. R. R. Tolkien', path="$['store']['book'][3]['author']")]
# ['Nigel Rees', 'Evelyn Waugh', 'Herman Melville', 'J. R. R. Tolkien']
# ["$['store']['book'][0]['author']", "$['store']['book'][1]['author']", "$['store']['book'][2]['author']", "$['store']['book'][3]['author']"]

```

`JsonPathResult` has the following attributes:

- data: the found value
- path: the path to the found value

`JsonPathResult` can't be constructed from Python; it is only returned by `Finder.find()`.

## Caveats

The current implementation is cloning the original `PyObject` data when converting it to the serde `Value`.
It happens each time you're creating a new `Finder` instance. Try to reuse the same `Finder` instance for querying if it's possible.

Also, It has yet another consequence demonstrated in the following example:

```python
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

## Development

Clone the repository.

Update maturin if needed:

```bash
pip install maturin -U
```

Add your changes, then run:

```bash
maturin develop
```


Update CI:

```bash
maturin generate-ci github > ./.github/workflows/CI.yml
```
