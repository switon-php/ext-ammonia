PHP_ARG_ENABLE([ammonia],
  [whether to enable ammonia],
  [AS_HELP_STRING([--enable-ammonia], [Enable the ammonia HTML sanitizer])],
  [yes])

if test "$PHP_AMMONIA" != "no"; then
  AC_PATH_PROG([CARGO], [cargo], [no])
  if test "$CARGO" = "no"; then
    AC_MSG_ERROR([cargo is required to build ammonia])
  fi
  PHP_SUBST([CARGO])
  dnl No C sources, so ext_srcdir is unset. Pass the fragment path directly.
  PHP_ADD_MAKEFILE_FRAGMENT([Makefile.frag], [.], [.])
fi
