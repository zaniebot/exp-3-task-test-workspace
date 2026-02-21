# Task Test Workspace

A test workspace for the agent task queue system.

## Testing

Run the tests with [pytest](https://docs.pytest.org/):
```bash
pip install pytest
pytest
```

## Scripts

### hello.py
A simple hello-world script with `main()` and `goodbye()` functions. Run it with:
```bash
python hello.py
```

Output:
```
Hello from the task queue!
Goodbye from the task queue!
```

To greet someone by name, use the `--name` flag:
```bash
python hello.py --name Alice
```
This will print `Hello, Alice!` instead of the default message.
