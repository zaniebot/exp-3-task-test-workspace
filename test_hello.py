import io
import sys

from hello import main


def test_main_output(capsys):
    """Test that main() prints the expected greeting."""
    main()
    captured = capsys.readouterr()
    assert captured.out == "Hello from the task queue!\n"


def test_main_no_stderr(capsys):
    """Test that main() produces no stderr output."""
    main()
    captured = capsys.readouterr()
    assert captured.err == ""
