from tests import Tests


class TestRunner:
    def __init__(self) -> None:
        self.tests = Tests()
        pass

    def get_lastTests(self) -> None:
        pass

    def run_tests(self) -> None:
        self.tests.get_top_words()
        pass
