import sys
from dataclasses import dataclass
import time
import re
import requests
import math


# --- CLIENT ---
headers = {
    "User-Agent": "Track/0.1.0 (github.com/MiniGun1239/Track)"
}

# --- DATA ---
@dataclass
class PlaneData:
    callsign: str
    registration: str
    type: str

    altitude: float
    ground_speed: float

    lat: float
    lon: float

@dataclass
class AirportData:
    name: str
    iata: str
    icao: str

    country: str
    location: str

    altitude: float
    lat: float
    lon: float


# --- Funcs ---

def get_callsign():
    callsign_pattern = r"^[A-Z]{3}\d{1,4}[A-Z]{0,2}$"

    while True:
        callsign = input("Enter callsign: ").strip().upper()

        if re.match(callsign_pattern, callsign):
            break
        else:
            print("Invalid callsign")

    return callsign


# gets data of the planes using this callsign
def get_flight(callsign):
    url = f"https://api.adsb.lol/v2/callsign/{callsign}"

    try:
        response = requests.get(url, headers=headers, timeout=5)

        if 200 <= response.status_code < 300:
            return response.json()
        else:
            print("get_flight_error")
            print(response.status_code)
            exit(1)
    except requests.exceptions.Timeout:
        print("request timed out, the api is ignoring us")
        exit(1)
    except requests.exceptions.RequestException as e:
        print(e)
        exit(1)


# gets the route of planes using this callsign
def get_route(callsign):
    leading_filepath = callsign[:2]

    url = f"https://vrs-standing-data.adsb.lol/routes/{leading_filepath}/{callsign}.json"

    try:
        response = requests.get(url, headers=headers, timeout=5)

        if 200 <= response.status_code < 300:
            return response.json()
        else:
            print("get_route error")
            print(response.status_code)
            exit(1)
    except requests.exceptions.Timeout:
        print("request timed out, the api is ignoring us")
        exit(1)
    except requests.exceptions.RequestException as e:
        print(e)
        exit(1)


# smth smth comment
def getData(
        plane_data, depAirport_data, destAirport_data
):
    plane = PlaneData(
        callsign=plane_data.get("flight"),
        registration=plane_data.get("r"),
        type=plane_data.get("t"),
        altitude=plane_data.get("alt_baro"),
        ground_speed=plane_data.get("gs"),
        lat=plane_data.get("lat"),
        lon=plane_data.get("lon"),
    )

    depAirport = AirportData(
        name=depAirport_data.get("name"),
        iata=depAirport_data.get("iata"),
        icao=depAirport_data.get("icao"),
        country=depAirport_data.get("countryiso2"),
        location=depAirport_data.get("location"),
        altitude=depAirport_data.get("altitude"),
        lat=depAirport_data.get("lat"),
        lon=depAirport_data.get("lon"),
    )

    destAirport = AirportData(
        name=destAirport_data.get("name"),
        iata=destAirport_data.get("iata"),
        icao=destAirport_data.get("icao"),
        country=destAirport_data.get("countryiso2"),
        location=destAirport_data.get("location"),
        altitude=destAirport_data.get("altitude"),
        lat=destAirport_data.get("lat"),
        lon=destAirport_data.get("lon"),
    )

    return plane, depAirport, destAirport


# haversine formula
def haversine(
        lat1: float, lon1: float,
        lat2: float, lon2: float
) -> float:
    earthRadius_KM = 6371.0

    phi1 = math.radians(lat1)
    phi2 = math.radians(lat2)
    dphi = math.radians(lat2 - lat1)
    dlambda = math.radians(lon2 - lon1)

    a = (math.sin(dphi / 2) ** 2 +
         math.cos(phi1) * math.cos(phi2) * math.sin(dlambda / 2) ** 2)

    c = 2 * math.atan2(math.sqrt(a), math.sqrt(1 - a))

    return earthRadius_KM * c

def getDistanceRatio(
        dep_lat: float, dep_lon: float,
        dest_lat: float, dest_lon: float,
        plane_lat: float, plane_lon: float
) -> float:
    airportDistance = haversine(
        dep_lat, dep_lon, dest_lat, dest_lon
    )

    planeDestDistance = haversine(
        plane_lat, plane_lon, dest_lat, dest_lon
    )

    return max(0, min(1 - (planeDestDistance / airportDistance), 1))


def outputShi(
        plane: PlaneData,
        depAirport: AirportData, destAirport: AirportData,
        travelRemaining: float, isFirst: bool
):
    if not isFirst:
        sys.stdout.write("\x1b[5A\x1b[J")
        sys.stdout.flush()

    bar_length = 50
    filled_blocks = int(round(bar_length * travelRemaining))
    remaining_blocks = bar_length - filled_blocks

    filled_track = "█" * filled_blocks
    remaining_track  = " " * remaining_blocks

    spaces = bar_length - (len(depAirport.location) + len(destAirport.location) + 4 + 8 + 4 + 8) + 8

    spaces = max(1, spaces)

    sys.stdout.write(f"Callsign: {plane.callsign} | Altitude: {plane.altitude} | Speed: {plane.ground_speed}\n")
    sys.stdout.write(f"Type: {plane.type} | Tail: {plane.registration}\n")
    sys.stdout.write(f"  [{filled_track}{remaining_track}] {(travelRemaining * 100):.2f}%\n")
    sys.stdout.write(f"{depAirport.location} - {depAirport.icao} ({depAirport.country}){" " * spaces}{destAirport.location} - {destAirport.icao} ({destAirport.country})\n")

    sys.stdout.flush()


# main, muehehehehehe
def main():
    # commented for testing
    # callsign = get_callsign()

    callsign = "UAE353"
    isFirst = True

    while True:
        # data of planes using this specific callsign
        flight_data = get_flight(callsign)

        # data of the actual plane
        plane_data = flight_data["ac"][0]

        route_info = get_route(callsign)

        depAirport_data  = route_info["_airports"][0]
        destAirport_data = route_info["_airports"][1]

        plane, depAirport, destAirport = getData(plane_data, depAirport_data, destAirport_data)

        travelRemaining = getDistanceRatio(
            depAirport.lat, depAirport.lon,
            destAirport.lat, destAirport.lon,
            plane.lat, plane.lon
        )

        outputShi(
            plane=plane,
            depAirport=depAirport, destAirport=destAirport,
            travelRemaining=travelRemaining, isFirst=isFirst)

        isFirst = False

        time.sleep(10)

if __name__ == "__main__":
    main()

