from pythoncurrent import submit


def test_submit():
  def task():
    print("Executing task!!")

  submit(task)
  assert True

def test_say_hello():
  pass # say_hello()

if '__main__' == __name__:
  test_submit()
