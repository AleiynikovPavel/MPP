import os


N = 1000
sum1 = 0
sum2 = 0
sum3 = 0
for i in range(0, N):
    stream = os.popen('bash run.sh')
    output = stream.read()
    sum1 += int(output.split('\n')[0][22:-2])
    sum2 += int(output.split('\n')[2][25:-2])
    sum3 += int(output.split('\n')[4][43:-2])

print('avg no par    ', sum1/N)
print('avg MAX thread', sum2/N)
print('avg 2 thread  ', sum3/N)
