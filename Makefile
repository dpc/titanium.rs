RS_CPP_SRCS := $(shell find src -name '*.rs.cpp')
RS_CPP_OUTS := $(RS_CPP_SRCS:.rs.cpp=_auto.rs)
CLEAN += $(RS_CPP_OUTS)

%_auto.rs: %.rs.cpp
	$(CC) -E -P $< -o $@

.PHONY: update
update: $(RS_CPP_OUTS)
