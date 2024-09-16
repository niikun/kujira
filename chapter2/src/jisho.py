import click

@click.command()
@click.argument('text')
def search(text):
    click.echo(f'Searching for {text}')
    dicfile = "ejdict-hand-utf8.txt"
    
    try:
        with open(dicfile, "r", encoding="utf-8") as f:
            found = False
            for line in f:
                if text.lower() in line.lower():
                    click.echo(line.strip())
                    found = True
            if not found:
                click.echo("No results found.")
    except FileNotFoundError:
        click.echo(f"Dictionary file '{dicfile}' not found.")

if __name__ == "__main__":
    search()