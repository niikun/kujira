import os
import click

@click.command()
@click.option("--path", default="./",help="Root directory to start searching for files")
@click.option("--name", default="", help="Name of the file to search for")
def find_file(path:str,name:str)->bool:
    found = False
    for dirpath,dirnames,filenames in os.walk(path):
        for file in filenames:
            # print(os.path.join(dirpath,file))
            if file == name:
                click.echo("True") 
                found = True
                break
    if not found:
        click.echo("False")


if __name__ == "__main__":
    find_file()
