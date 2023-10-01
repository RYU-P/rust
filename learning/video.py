def createClip(imageFile, audioFile, outputFile):
    """Creates a video clip from an image and an audio file."""
    clip = ImageClip(imageFile)
    clip = clip.set_audio(AudioFileClip(audioFile))
    clip.write_videofile(outputFile, fps=24)

# Path: video.py
def createTextClip(text, outputFile):
    """Creates a video clip from a text."""
    clip = TextClip(text, font="Arial", fontsize=24, color="white")
    clip.write_videofile(outputFile, fps=24)

def main():
    """Main function."""
    # Create a video clip from an image and an audio file.
    createClip("image.jpg", "audio.mp3", "output.mp4")

    # Create a video clip from a text.
    createTextClip("Hello world!", "output.mp4")
    clips[]
    clips
    for i in range(nk)