
============
Installation
============

pip
---

.. code-block:: console

   $ pip install startinpy


development
-----------

1. install `Rust <https://www.rust-lang.org/>`_ (v1.39+)
2. install `maturin <https://github.com/PyO3/maturin>`_ 
3. compile the rust code and build the Python bindings:

  .. code-block:: console 

    $ maturin develop

4. move to another folder, and this shouldn't return any error:
   
  .. code-block:: console

    $ python
    $ import startinpy