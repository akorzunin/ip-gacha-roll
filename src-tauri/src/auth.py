import os
import requests
import hashlib

cookies_current = None
session = requests.session()


def keen_auth(login, passw):
    response = keen_request(ip_addr, "auth")

    if response.status_code == 401:
        md5 = login + ":" + response.headers["X-NDM-Realm"] + ":" + passw
        md5 = hashlib.md5(md5.encode("utf-8"))
        sha = response.headers["X-NDM-Challenge"] + md5.hexdigest()
        sha = hashlib.sha256(sha.encode("utf-8"))
        print(sha.hexdigest())
        response = keen_request(
            ip_addr, "auth", {"login": login, "password": sha.hexdigest()}
        )
        if response.status_code == 200:
            return True
    elif response.status_code == 200:
        return True
    else:
        return False


def keen_request(ip_addr, query, post=None):  # отправка запросов на роутер

    global session

    url = "http://" + ip_addr + "/" + query

    if post:
        return session.post(url, json=post)
    else:
        return session.get(url)


if __name__ == "__main__":
    ip_addr = "192.168.1.1"
    login = "admin"
    # passw = os.getenv('ROUTER_PASS')
    passw = "admin"
    if keen_auth(login, passw):
        response = keen_request(ip_addr, "rci/show/interface/WifiMaster0")

    print(response.text)
