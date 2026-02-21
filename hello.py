import argparse


def main():
    parser = argparse.ArgumentParser(description="A simple hello-world script.")
    parser.add_argument("--name", type=str, default=None, help="Name to greet")
    args = parser.parse_args()

    if args.name:
        print(f"Hello, {args.name}!")
    else:
        print("Hello from the task queue!")


if __name__ == "__main__":
    main()
