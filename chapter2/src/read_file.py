import click

@click.command()
@click.argument("filename")
def read_file(filename):
    with open(filename,"r")as f:
        text = f.read()
    total = 0
    for t in text.split("\n"):
        print(t)
        try:
            total += int(t)
        except ValueError:
            pass
    print(total)

if __name__ == "__main__":
    read_file()