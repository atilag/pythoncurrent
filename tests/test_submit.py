from pythoncurrent import submit, say_hello


def test_submit():
  def task(f64):
    print("Executing task!! {}".format(f64))
    return 69.0

  submit(task)
  assert True

def test_say_hello():
  say_hello()

if '__main__' == __name__:
  test_say_hello()
  test_submit()
