def setup():
    with open("path.txt", "w+") as my_file:
        my_file.write("")
    import nltk

    nltk.download("punkt")
    nltk.download("averaged_perceptron_tagger")
