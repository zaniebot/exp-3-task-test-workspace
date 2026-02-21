import argparse
import logging

logger = logging.getLogger(__name__)


def main():
    parser = argparse.ArgumentParser(description="A simple hello-world script.")
    parser.add_argument("--verbose", action="store_true", help="Enable debug logging")
    parser.add_argument("--name", type=str, default=None, help="Name to greet")
    args = parser.parse_args()

    logging.basicConfig(
        level=logging.DEBUG if args.verbose else logging.INFO,
        format="%(asctime)s %(name)s %(levelname)s: %(message)s",
    )

    logger.info("main() is running")
    if args.name:
        print(f"Hello, {args.name}!")
    else:
        print("Hello from the task queue!")


def goodbye():
    print("Goodbye from the task queue!")


if __name__ == "__main__":
    main()
    goodbye()
