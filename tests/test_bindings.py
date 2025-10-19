import pytest
from jsonpath_rust_bindings import Finder
from jsonpath_rust_bindings import JsonPathResult


@pytest.fixture
def sample_data() -> dict:
    return {
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


def test_sanity(sample_data):
    finder = Finder(sample_data)
    res = finder.find("$..book[?(@.price<=$.expensive)]")
    assert len(res) == 2
    assert isinstance(res[0], JsonPathResult)


def test_exceptions(sample_data):
    finder = Finder(sample_data)

    with pytest.raises(ValueError):
        finder.find("fail")


def test_repr(sample_data):
    finder = Finder(sample_data)
    result = str(finder.find("$.store.bicycle.color")[0])
    assert (
        result
        == """JsonPathResult(data='red', path="$['store']['bicycle']['color']")"""
    )


def test_overflow():
    big_number = 18446744005107584948
    f = Finder({"test": big_number})
    res = f.find('$.test')[0].data
    assert res == big_number


QUERIES = [
    "$.store.book[*].author",
    "$..book[?(@.isbn)]",
    "$.store.*",
    "$..author",
    "$.store..price",
    "$..book[2]",
    "$..book[-2]",
    "$..book[0,1]",
    "$..book[:2]",
    "$..book[1:2]",
    "$..book[-2:]",
    "$..book[2:]",
    "$.store.book[?(@.price<10)]",
    "$..book[?(@.price<=$.expensive)]",
    "$..*",
    r'$..book[?match(@.author, "(?i).*rees.*")]',
]

@pytest.mark.parametrize("query", QUERIES, ids=QUERIES)
def test_smoke(sample_data, query):
    f = Finder(sample_data)

    results = f.find(query)
    assert isinstance(results, list)
    assert all(isinstance(r, JsonPathResult) for r in results)

    assert [r.data for r in results] == f.find_data(query)
    assert [r.path for r in results] == f.find_absolute_path(query)


def test_authors_data_and_paths(sample_data):
    f = Finder(sample_data)
    results = f.find("$.store.book[*].author")
    data, paths = _extract(results)

    assert data == [
        "Nigel Rees",
        "Evelyn Waugh",
        "Herman Melville",
        "J. R. R. Tolkien",
    ]
    assert paths == [
        "$['store']['book'][0]['author']",
        "$['store']['book'][1]['author']",
        "$['store']['book'][2]['author']",
        "$['store']['book'][3]['author']",
    ]

def test_bicycle_color_value_and_path(sample_data):
    f = Finder(sample_data)
    results = f.find("$.store.bicycle.color")
    data, paths = _extract(results)

    assert data == ["red"]
    assert paths == ["$['store']['bicycle']['color']"]


def _extract(results):
    assert all(isinstance(r, JsonPathResult) for r in results)
    return [r.data for r in results], [r.path for r in results]
