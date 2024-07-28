import unittest
from parliament import Context

func = __import__("func")


class DummyRequest:
    def __init__(self, descriptions):
        self.descriptions = descriptions

    @property
    def args(self):
        return dict(descriptions=self.descriptions)


class TestFunc(unittest.TestCase):
    # noinspection PyTypeChecker
    def test_func(self):
        result, code = func.main(Context(DummyRequest('flame,confused')))
        expected = """{"flame": "('fire', '🔥')", "confused": "('confused_face', '😕')"}"""
        self.assertEqual(expected, result)
        self.assertEqual(code, 200)


if __name__ == "__main__":
    unittest.main()
