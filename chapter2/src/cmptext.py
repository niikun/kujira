import click

@click.command()
@click.argument("file1")
@click.argument("file2")
def compare_text(file1,file2):
    with open(file1,"r") as f:
        text1 = f.read()
    with open(file2,"r") as f:
        text2 = f.read()
    

    if text1 == text2:
        print("file1 and file2 are the same")
    # for t1,t2 in zip(text1,text2):
    #     if t1 != t2:
    #         print("file1 and file2  are different")

    # print("file1 and file2 are the same")
    
if __name__ == "__main__":

    compare_text()