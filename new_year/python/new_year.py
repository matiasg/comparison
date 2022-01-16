import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger()


def catalan(n):
    if n == 1:
        yield []
    elif n == 2:
        yield [1]
    else:
        ret = []
        for b in range(1, n):
            for c1 in catalan(b):
                for c2 in catalan(n - b):
                    yield c1 + [i + 1 for i in c2] + [1]


OPERATORS = [
    (lambda a, b: a + b),
    (lambda a, b: a - b),
    (lambda a, b: a * b),
]


def operations(n):
    if n == 1:
        return [[]]
    return [
        [f] + rest
        for f in OPERATORS
        for rest in operations(n - 1)
    ]


def apply(ops, order):
    l = list(range(len(ops) + 1, 0, -1))
    for (op, ord_) in zip(ops, order):
        res = op(l[ord_ - 1], l[ord_])
        l[ord_-1:ord_+1] = [res]
    return l[0]


def apply_all(all_ops, orders):
    ret = set()
    last_size = 0
    for j, order in enumerate(orders):
        for i, ops in enumerate(all_ops):
            res = apply(ops, order)
            if res > 0:
                ret.add(res)
                current_size = len(ret)
                if current_size > last_size:
                    logger.debug("order %s, ops %s, result %s, total reached %s",
                                 j, i, res, len(ret))
                    last_size = current_size
    return ret


if __name__ == '__main__':
    size = int(input("how many numbers? "))
    all_ops = operations(size)
    orders = catalan(size)
    results = apply_all(all_ops, orders)
    print(f"got {len(results)} results")
    logger.debug("all results: %s", sorted(results))
