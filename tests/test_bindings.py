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


def test_all_queries(sample_data):
    queries = [
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
        # "$..book[?@.author ~= '(?i)REES']",
        "$..*",
    ]
    queries_results = [
        """[JsonPathResult(data='Nigel Rees', path="$['store']['book'][0]['author']"), 
JsonPathResult(data='Evelyn Waugh', path="$['store']['book'][1]['author']"), 
JsonPathResult(data='Herman Melville', path="$['store']['book'][2]['author']"), 
JsonPathResult(data='J. R. R. Tolkien', path="$['store']['book'][3]['author']")]""",
        """[JsonPathResult(data={'author': 'Herman Melville', 'category': 'fiction', 'isbn': '0-553-21311-3', 'price': 8.99, 'title': 'Moby Dick'}, 
path="$['store']['book'][2]"), 
JsonPathResult(data={'author': 'J. R. R. Tolkien', 'category': 'fiction', 'isbn': '0-395-19395-8', 'price': 22.99, 'title': 'The Lord of the Rings'}, 
path="$['store']['book'][3]")]""",
        """[JsonPathResult(data={'color': 'red', 'price': 19.95}, path="$['store']['bicycle']"), 
JsonPathResult(data=[{'author': 'Nigel Rees', 'category': 'reference', 'price': 8.95, 'title': 'Sayings of the Century'}, 
{'author': 'Evelyn Waugh', 'category': 'fiction', 'price': 12.99, 'title': 'Sword of Honour'}, 
{'author': 'Herman Melville', 'category': 'fiction', 'isbn': '0-553-21311-3', 'price': 8.99, 'title': 'Moby Dick'}, 
{'author': 'J. R. R. Tolkien', 'category': 'fiction', 'isbn': '0-395-19395-8', 'price': 22.99, 'title': 'The Lord of the Rings'}], 
path="$['store']['book']")]""",
        """[JsonPathResult(data='Nigel Rees', path="$['store']['book'][0]['author']"), 
JsonPathResult(data='Evelyn Waugh', path="$['store']['book'][1]['author']"), 
JsonPathResult(data='Herman Melville', path="$['store']['book'][2]['author']"), 
JsonPathResult(data='J. R. R. Tolkien', path="$['store']['book'][3]['author']")]""",
        """[JsonPathResult(data=19.95, path="$['store']['bicycle']['price']"), 
JsonPathResult(data=8.95, path="$['store']['book'][0]['price']"), 
JsonPathResult(data=12.99, path="$['store']['book'][1]['price']"), 
JsonPathResult(data=8.99, path="$['store']['book'][2]['price']"), 
JsonPathResult(data=22.99, path="$['store']['book'][3]['price']")]""",
        """[JsonPathResult(data={'author': 'Herman Melville', 'category': 'fiction', 'isbn': '0-553-21311-3', 'price': 8.99, 'title': 'Moby Dick'}, 
path="$['store']['book'][2]")]""",
        """[JsonPathResult(data={'author': 'Herman Melville', 'category': 'fiction', 'isbn': '0-553-21311-3', 'price': 8.99, 'title': 'Moby Dick'}, 
path="$['store']['book'][2]")]""",
        """[JsonPathResult(data={'author': 'Nigel Rees', 'category': 'reference', 'price': 8.95, 'title': 'Sayings of the Century'}, 
path="$['store']['book'][0]"), 
JsonPathResult(data={'author': 'Evelyn Waugh', 'category': 'fiction', 'price': 12.99, 'title': 'Sword of Honour'}, path="$['store']['book'][1]")]""",
        """[JsonPathResult(data={'author': 'Nigel Rees', 'category': 'reference', 'price': 8.95, 'title': 'Sayings of the Century'}, 
path="$['store']['book'][0]"), 
JsonPathResult(data={'author': 'Evelyn Waugh', 'category': 'fiction', 'price': 12.99, 'title': 'Sword of Honour'}, path="$['store']['book'][1]")]""",
        """[JsonPathResult(data={'author': 'Evelyn Waugh', 'category': 'fiction', 'price': 12.99, 'title': 'Sword of Honour'}, 
path="$['store']['book'][1]")]""",
        """[JsonPathResult(data={'author': 'Herman Melville', 'category': 'fiction', 'isbn': '0-553-21311-3', 'price': 8.99, 'title': 'Moby Dick'}, 
path="$['store']['book'][2]"), 
JsonPathResult(data={'author': 'J. R. R. Tolkien', 'category': 'fiction', 'isbn': '0-395-19395-8', 'price': 22.99, 'title': 'The Lord of the Rings'}, 
path="$['store']['book'][3]")]""",
        """[JsonPathResult(data={'author': 'Herman Melville', 'category': 'fiction', 'isbn': '0-553-21311-3', 'price': 8.99, 'title': 'Moby Dick'}, 
path="$['store']['book'][2]"), 
JsonPathResult(data={'author': 'J. R. R. Tolkien', 'category': 'fiction', 'isbn': '0-395-19395-8', 'price': 22.99, 'title': 'The Lord of the Rings'}, 
path="$['store']['book'][3]")]""",
        """[JsonPathResult(data={'author': 'Nigel Rees', 'category': 'reference', 'price': 8.95, 'title': 'Sayings of the Century'}, 
path="$['store']['book'][0]"), 
JsonPathResult(data={'author': 'Herman Melville', 'category': 'fiction', 'isbn': '0-553-21311-3', 'price': 8.99, 'title': 'Moby Dick'}, 
path="$['store']['book'][2]")]""",
        """[JsonPathResult(data={'author': 'Nigel Rees', 'category': 'reference', 'price': 8.95, 'title': 'Sayings of the Century'}, 
path="$['store']['book'][0]"), 
JsonPathResult(data={'author': 'Herman Melville', 'category': 'fiction', 'isbn': '0-553-21311-3', 'price': 8.99, 'title': 'Moby Dick'}, 
path="$['store']['book'][2]")]""",
        """[JsonPathResult(data=10, path="$['expensive']"), 
JsonPathResult(data={'bicycle': {'color': 'red', 'price': 19.95}, 
'book': [{'author': 'Nigel Rees', 'category': 'reference', 'price': 8.95, 'title': 'Sayings of the Century'}, 
{'author': 'Evelyn Waugh', 'category': 'fiction', 'price': 12.99, 'title': 'Sword of Honour'}, 
{'author': 'Herman Melville', 'category': 'fiction', 'isbn': '0-553-21311-3', 'price': 8.99, 'title': 'Moby Dick'}, 
{'author': 'J. R. R. Tolkien', 'category': 'fiction', 'isbn': '0-395-19395-8', 'price': 22.99, 'title': 'The Lord of the Rings'}]}, 
path="$['store']"), 
JsonPathResult(data={'color': 'red', 'price': 19.95}, path="$['store']['bicycle']"), 
JsonPathResult(data=[{'author': 'Nigel Rees', 'category': 'reference', 'price': 8.95, 'title': 'Sayings of the Century'}, 
{'author': 'Evelyn Waugh', 'category': 'fiction', 'price': 12.99, 'title': 'Sword of Honour'}, 
{'author': 'Herman Melville', 'category': 'fiction', 'isbn': '0-553-21311-3', 'price': 8.99, 'title': 'Moby Dick'}, 
{'author': 'J. R. R. Tolkien', 'category': 'fiction', 'isbn': '0-395-19395-8', 'price': 22.99, 'title': 'The Lord of the Rings'}], 
path="$['store']['book']"), 
JsonPathResult(data='red', path="$['store']['bicycle']['color']"), 
JsonPathResult(data=19.95, path="$['store']['bicycle']['price']"), 
JsonPathResult(data={'author': 'Nigel Rees', 'category': 'reference', 'price': 8.95, 'title': 'Sayings of the Century'}, 
path="$['store']['book'][0]"), 
JsonPathResult(data={'author': 'Evelyn Waugh', 'category': 'fiction', 'price': 12.99, 'title': 'Sword of Honour'}, path="$['store']['book'][1]"), 
JsonPathResult(data={'author': 'Herman Melville', 'category': 'fiction', 'isbn': '0-553-21311-3', 'price': 8.99, 'title': 'Moby Dick'}, 
path="$['store']['book'][2]"), 
JsonPathResult(data={'author': 'J. R. R. Tolkien', 'category': 'fiction', 'isbn': '0-395-19395-8', 'price': 22.99, 'title': 'The Lord of the Rings'}, 
path="$['store']['book'][3]"), 
JsonPathResult(data='Nigel Rees', path="$['store']['book'][0]['author']"), 
JsonPathResult(data='reference', path="$['store']['book'][0]['category']"), 
JsonPathResult(data=8.95, path="$['store']['book'][0]['price']"), 
JsonPathResult(data='Sayings of the Century', path="$['store']['book'][0]['title']"), 
JsonPathResult(data='Evelyn Waugh', path="$['store']['book'][1]['author']"), 
JsonPathResult(data='fiction', path="$['store']['book'][1]['category']"), 
JsonPathResult(data=12.99, path="$['store']['book'][1]['price']"), 
JsonPathResult(data='Sword of Honour', path="$['store']['book'][1]['title']"), 
JsonPathResult(data='Herman Melville', path="$['store']['book'][2]['author']"), 
JsonPathResult(data='fiction', path="$['store']['book'][2]['category']"), 
JsonPathResult(data='0-553-21311-3', path="$['store']['book'][2]['isbn']"), 
JsonPathResult(data=8.99, path="$['store']['book'][2]['price']"), 
JsonPathResult(data='Moby Dick', path="$['store']['book'][2]['title']"), 
JsonPathResult(data='J. R. R. Tolkien', path="$['store']['book'][3]['author']"), 
JsonPathResult(data='fiction', path="$['store']['book'][3]['category']"), 
JsonPathResult(data='0-395-19395-8', path="$['store']['book'][3]['isbn']"), 
JsonPathResult(data=22.99, path="$['store']['book'][3]['price']"), 
JsonPathResult(data='The Lord of the Rings', path="$['store']['book'][3]['title']")]""",
    ]
    f = Finder(sample_data)
    res = []
    res_data = []
    res_absolute_path = []
    for query, queries_result in zip(queries, queries_results):
        temp_res = f.find(query)
        res.extend(temp_res)
        res_data.extend(f.find_data(query))
        res_absolute_path.extend(f.find_absolute_path(query))
        assert str(temp_res) == queries_result.replace("\n", "")
        assert [r.data for r in res] == res_data
        assert [r.path for r in res] == res_absolute_path
        # print(query)
        # print(f.find(query), '\n')
        # print('----------')


def test_overflow():
    big_number = 18446744005107584948
    f = Finder({"test": big_number})
    res = f.find('$.test')[0].data
    assert res == big_number
