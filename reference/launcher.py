import os
import threading

# Replace 'file_name.exe' with the actual name of the executable file you want to launch
file_name = 'C:\\Tools\\SteamCMD\\steamcmd.exe'

parameters = '+login anonymous'

# Check if the file exists
if os.path.exists(file_name):
    # Define a function to launch the file with the specified parameters
    def launch_file():
        os.system(f"{file_name} {parameters}")

    # Create a new thread to run the launch_file function
    thread = threading.Thread(target=launch_file)

    # Start the thread
    thread.start()

    print("File launched on separate thread.")
else:
    print(f"Error: the file '{file_name}' does not exist.")
