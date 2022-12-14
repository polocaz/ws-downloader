import subprocess
import time

# Path to the executable you want to launch
exe_path = "C:\\Tools\\SteamCMD\\steamcmd.exe"

# List of parameters to pass to the executable
params = ["+login", "anonymous"]

print ("Before starting")
# Launch the executable with the specified parameters and capture the output
# result = subprocess.run([exe_path] + params, capture_output=True)
result = subprocess.run([exe_path], capture_output=True)

print (result)
# Wait until the "Success" message appears in the output
while "Success" not in result.stdout.decode():
    print(result.stdout.decode())
    time.sleep(1)

# The "Success" message has appeared, so print a success message
print("Logged in successfully!")
