def idx(v):
    return ord(v) - ord("a")


class TrieNode:
    def __init__(self) -> None:
        self.children: list[TrieNode] = [None for _ in range(26)]
        self.counter = [0 for _ in range(26)]
        self.end = False

    def print_tree(self, spaces: str = ""):
        print(spaces + str(self))
        for i in range(26):
            if self.children[i]:
                self.children[i].print_tree(spaces + "\t")

    def __str__(self) -> str:
        return f"{[(chr(ord('a')+ i), self.counter[i], self.children[i].end) for i in range(26) if self.children[i]]}"


class Trie:
    def __init__(self) -> None:
        self.root = TrieNode()

    def insert(self, text: str):
        curr_node = self.root
        for letter in text:
            index = idx(letter)
            if curr_node.children[index] is None:
                curr_node.children[index] = TrieNode()
            curr_node.counter[index] += 1
            curr_node = curr_node.children[index]
        curr_node.end = True

    def count(self, text, start=0):
        # print("count", text[start:])
        curr_node = self.root
        ans = 0
        for i in range(start, len(text)):
            letter = text[i]
            index = idx(letter)
            if curr_node.end:
                ans += self.count(text, i)
            if curr_node.children[index] is None:
                return ans
            curr_node = curr_node.children[index]
        if curr_node.end:
            ans += 1
        return ans

    def print_tree(self) -> str:
        return str(self.root.print_tree())


text = input().strip()
k = int(input())

root = Trie()

for _ in range(k):
    word = input().strip()
    # print("insert", word)
    root.insert(word)


root.print_tree()

print(root.count(text))
