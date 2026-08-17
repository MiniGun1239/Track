import logging
import math
import re
import sys
import time
from dataclasses import dataclass

import requests

# --- CONFIGURATION ---
UPDATE_INTERVAL = 10
MAX_FLIGHT_DATA_ERRORS = 3
DEBUG = False

logging.basicConfig(
    level=logging.DEBUG if DEBUG else logging.INFO,
    format="[%(levelname)s] %(message)s"
)

# Shared HTTP Session
SESSION = requests.Session()
SESSION.headers.update({
    "User-Agent": "Track/1.2.0 (github.com/MiniGun1239/Track)"
})


# --- DATA STUFF ---
@dataclass
class PlaneData:
    callsign: str
    registration: str
    type: str

    altitude: float
    ground_speed: float

    lat: float
    lon: float

    @classmethod
    def from_dict(cls, data: dict) -> "PlaneData":
        return cls(
            callsign=data.get("flight"),
            registration=data.get("r"),
            type=data.get("t"),
            altitude=data.get("alt_baro"),
            ground_speed=data.get("gs"),
            lat=data.get("lat"),
            lon=data.get("lon"),
        )


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

    @classmethod
    def from_dict(cls, data: dict) -> "AirportData":
        return cls(
            name=data.get("name"),
            iata=data.get("iata"),
            icao=data.get("icao"),
            country=data.get("countryiso2"),
            location=data.get("location"),
            altitude=data.get("altitude"),
            lat=data.get("lat"),
            lon=data.get("lon"),
        )


# --- API CALLERS ---
def get_callsign() -> str:
    """
    Prompt user for a valid ICAO callsign.
    """

    callsign_pattern = re.compile(r"^[A-Z]{3}\d{1,4}[A-Z]{0,2}$")

    while True:
        callsign = input("Enter callsign: ").strip().upper()
        if callsign_pattern.match(callsign):
            logging.debug("Validated callsign: %s", callsign)
            return callsign
        print("Invalid callsign format. (e.g., AAL123, BAW4B)")


def get_flightData(callsign: str, isFirst: bool) -> [dict]:
    """
    Retrieve live flight telemetry from adsb.lol.
    """

    url = f"https://api.adsb.lol/v2/callsign/{callsign}"

    try:
        logging.debug("Fetching flight telemetry: %s", url)
        response = SESSION.get(url, timeout=5)

        if response.status_code == 200:
            logging.debug(f"Successfully received package")
            return response.json()
        elif response.status_code == 404:
            sys.exit("Error 404: Flight not found. Check if the callsign is correct.")
        sys.exit(f"API Error: HTTP {response.status_code}")

    except requests.exceptions.Timeout:
        if isFirst:
            print("Request timed out on initial attempt. Retrying...")
            return None
        logging.warning("Flight data fetch timed out.")
    except requests.exceptions.RequestException as err:
        sys.exit(f"Network error encountered: {err}")
    return None


def get_RouteData(callsign: str) -> dict:
    """
    Retrieve scheduled route information from adsb.lol
    """

    prefix = callsign[:2]
    url = f"https://vrs-standing-data.adsb.lol/routes/{prefix}/{callsign}.json"

    try:
        logging.debug(f"Fetching route data: {url}")
        response = SESSION.get(url, timeout=5)

        if response.status_code == 200:
            logging.debug(f"Successfully received package")
            return response.json()
        elif response.status_code == 404:
            sys.exit(
                "Error 404: Route not found. The flight may not have a published "
                "route or it is unlisted in the database."
            )
        sys.exit(f"Route API Error: HTTP {response.status_code}")

    except requests.exceptions.RequestException as err:
        sys.exit(f"Failed to fetch route data: {err}")


# --- CALCULATIONS ---
def haversine(
        lat1: float, lon1: float,
        lat2: float, lon2: float
) -> float:
    """
    Calculate the great-circle distance between two points in kilometers.
    """
    earthRadius_KM = 6371.0

    phi1, phi2 = math.radians(lat1), math.radians(lat2)
    dphi = math.radians(lat2 - lat1)
    dlambda = math.radians(lon2 - lon1)

    a = (
            math.sin(dphi / 2) ** 2
            + math.cos(phi1) * math.cos(phi2) * math.sin(dlambda / 2) ** 2
    )

    c = 2 * math.atan2(math.sqrt(a), math.sqrt(1 - a))

    return earthRadius_KM * c


def getProgress(
        dep_lat: float, dep_lon: float,
        dest_lat: float, dest_lon: float,
        plane_lat: float, plane_lon: float
) -> float:
    """
    Calculate journey progress as a float between 0.0 and 1.0.
    """

    total_distance = haversine(dep_lat, dep_lon, dest_lat, dest_lon)
    if total_distance == 0:
        return 0.0

    remaining_distance = haversine(plane_lat, plane_lon, dest_lat, dest_lon)
    progress = 1.0 - (remaining_distance / total_distance)
    return max(0.0, min(progress, 1.0))


# --- DISPLAY ---
def outputShi(
        plane: PlaneData,
        depAirport: AirportData,
        destAirport: AirportData,
        progress: float,
        is_first_frame: bool
) -> None:
    """
    Render terminal status panel and progress bar.
    """

    if not is_first_frame:
        # Move cursor up 4 lines and clear below
        sys.stdout.write("\x1b[4A\r\x1b[J")

    bar_length = 100
    filled_blocks = int(round(bar_length * progress))
    bar = "█" * filled_blocks + " " * (bar_length - filled_blocks)

    depStr = f"{depAirport.location} - {depAirport.icao} ({depAirport.country})"
    destStr = f"{destAirport.location} - {destAirport.icao} ({destAirport.country})"
    spacing = max(1, bar_length - (len(depStr) + len(destStr)) + 8)

    sys.stdout.write(
        f"Callsign: {plane.callsign or 'N/A'} | Altitude: {plane.altitude or 0} ft | Speed: {plane.ground_speed or 0} kts\n"
        f"Type: {plane.type or 'N/A'} | Tail: {plane.registration or 'N/A'} | Progress: {progress * 100:.2f}%\n"
        f"  [{bar}]\n"
        f"{depStr}{' ' * spacing}{destStr}\n"
    )
    sys.stdout.flush()


# main, muehehehehehe
def main():
    callsign = get_callsign()

    isFirst = True
    errorCount = 0
    routeData = None

    while True:
        flightData_payload = get_flightData(callsign, isFirst)
        if not flightData_payload:
            time.sleep(UPDATE_INTERVAL / 2)
            continue

        try:
            aircraftData = flightData_payload["ac"][0]
            errorCount = 0
        except (KeyError, IndexError):
            if isFirst:
                sys.exit(
                    "Flight info not found. The aircraft might be offline, "
                    "not transponding, or has already landed."
                )

            errorCount += 1
            if errorCount >= MAX_FLIGHT_DATA_ERRORS:
                sys.exit("Flight signal lost. Aircraft may have landed.")

            time.sleep(UPDATE_INTERVAL)
            continue

        if isFirst:
            routeData = get_RouteData(callsign)

        plane = PlaneData.from_dict(aircraftData)
        depAirport = AirportData.from_dict(routeData["_airports"][0])
        destAirport = AirportData.from_dict(routeData["_airports"][1])

        # Validate coordinates exist and are numeric
        verifyCoords = all(
            isinstance(val, (int, float)) for val in [
                depAirport.lat, depAirport.lon,
                destAirport.lat, destAirport.lon,
                plane.lat, plane.lon
            ]
        )

        if verifyCoords:
            progress = getProgress(
                depAirport.lat, depAirport.lon,
                destAirport.lat, destAirport.lon,
                plane.lat, plane.lon
            )
        else:
            if isFirst:
                sys.exit("Incomplete location data for aircraft or airports")
            time.sleep(UPDATE_INTERVAL)
            continue

        outputShi(plane, depAirport, destAirport, progress, isFirst)

        isFirst = False

        time.sleep(UPDATE_INTERVAL)


if __name__ == "__main__":
    main()

