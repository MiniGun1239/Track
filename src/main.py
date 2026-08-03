import requests
import json
import re

def main():
    callsign_pattern = r"^[A-Z]{3}\d{1,4}[A-Z]{0,2}$"

    while True:
        callsign = input("Enter callsign: ")

        if re.match(callsign_pattern, callsign):
            break
        else:
            print("Invalid callsign")

    