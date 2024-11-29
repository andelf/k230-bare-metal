import hashlib


with open("./k230_uboot_sd.img", 'rb') as f:
    f.seek(0x00100210)
    img = f.read(0x02fdb4)

l = len(img)

print('len', l, hex(l))

with open("./extract-fbl.img", 'wb') as f:
    f.write(img)



def sha256sum(data):
    sha256 = hashlib.sha256()
    sha256.update(data)
    return sha256.hexdigest()


print(sha256sum(img))
