import sys

sys.path.insert(0, "../../lib")
from lg_gen import emit, rng

r = rng(721)
NAMES = ["Alex", "John", "Mary", "Bob", "Lily", "David", "Celine", "Ethan", "Hanzo", "Fern",
         "Gabe", "Kevin", "Isa", "Zoe", "Quinn", "Olivia", "Max"]
DOMAINS = ["mail.com", "m.co", "x.org", "post.net"]


def email(pid, k, name):
    local = r.choice([name.lower(), name, name.lower() + "_"]) + str(pid) + r.choice(["", "_", "."]) + str(k)
    return local + "@" + r.choice(DOMAINS)


def person_accounts(pid, n_acc, pool_size, connected):
    """Accounts of one person. If `connected`, consecutive accounts share an email so they merge."""
    name = r.choice(NAMES)
    pool = sorted({email(pid, k, name) for k in range(pool_size)})
    accs = []
    for i in range(n_acc):
        size = r.randint(1, 9)
        emails = [r.choice(pool) for _ in range(size)]
        if connected and accs:
            emails[0] = r.choice(accs[-1][1:])
        accs.append([name] + emails)
    return accs


def build(people):
    accounts = [a for p in people for a in p]
    r.shuffle(accounts)
    for a in accounts:
        tail = a[1:]
        r.shuffle(tail)
        a[1:] = tail
    return accounts


def merge_brute(accounts):
    groups = [(a[0], set(a[1:])) for a in accounts]
    merged = True
    while merged:
        merged = False
        for i in range(len(groups)):
            for j in range(i + 1, len(groups)):
                if groups[i][1] & groups[j][1]:
                    groups[i][1].update(groups[j][1])
                    groups.pop(j)
                    merged = True
                    break
            if merged:
                break
    return sorted([name] + sorted(e) for name, e in groups)


def chain(n, name="Chain", pid=9000):
    """n accounts where account i shares one email with account i + 1: one long merge chain."""
    accs = [[name, f"c{pid}_{i}@mail.com", f"c{pid}_{i + 1}@mail.com"] for i in range(n)]
    r.shuffle(accs)
    return accs


emit([["Alex", "alex@mail.com"]])
emit([["Alex", "alex@mail.com"], ["Alex", "alex@mail.com"]])
emit([["Alex", "alex@mail.com", "alex@mail.com"]])
emit([["Alex", "a@m.co"], ["Alex", "b@m.co"], ["Alex", "a@m.co", "b@m.co"]])
emit([["Bob", "b2@m.co", "b1@m.co"], ["Bob", "b3@m.co"], ["Bob", "b1@m.co", "b3@m.co"]])
emit([["Ann", "z@m.co", "Z@m.co", "_a@m.co", "a_b@m.co", "ab@m.co", "a.b@m.co", "0a@m.co"]])
emit([["John", "j1@m.co"], ["John", "j2@m.co"], ["John", "j3@m.co"]])
emit([["Kim", "k0@m.co", "k1@m.co"], ["Kim", "k2@m.co", "k3@m.co"], ["Kim", "k4@m.co", "k5@m.co"], ["Kim", "k1@m.co", "k2@m.co"], ["Kim", "k5@m.co", "k3@m.co"]])
for _ in range(30):
    people = [person_accounts(pid, r.randint(1, 4), r.randint(1, 8), r.random() < 0.6)
              for pid in range(r.randint(1, 6))]
    emit(build(people))
for _ in range(10):
    people = [person_accounts(pid, r.randint(1, 10), r.randint(2, 30), r.random() < 0.7)
              for pid in range(r.randint(10, 60))]
    emit(build(people))
emit(chain(30))

# Large cases (1000 accounts). Long merge chains make repeated pairwise merging very slow.
emit(chain(1000))
emit(chain(500, "Ann", 1) + chain(500, "Ann", 2))
emit(build([person_accounts(pid, 1, 9, False) for pid in range(1000)]))
emit(build([person_accounts(pid, 10, 40, True) for pid in range(100)]))
emit(build([person_accounts(0, 1000, 3000, True)]))
emit(build([person_accounts(pid, 50, 200, True) for pid in range(20)]))
