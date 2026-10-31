#!/bin/sh
# SPDX-License-Identifier: Apache-2.0
#
# melt-pot bring-up HIL, run on the board as root:
#
#   sh run.sh <imx93|stm32mp25> [dist-dir]
#
# Loads the CM33 firmware through remoteproc, runs the A-core application for
# DURATION seconds (default 15), then checks the `HIL A<n>` / `HIL M<n>` stage
# lines both binaries print. The binaries do the real checking and print a
# trace when a stage stalls; this script only loads, runs and summarizes.
#
# dist-dir is the copied `dist-bringup/` tree (default: the current directory),
# holding firmware/cm33.elf and app/<application>.

set -u

BOARD="${1:-}"
DIST="${2:-.}"
DURATION="${DURATION:-15}"
FW_NAME=consortium-cm33.elf

case "$BOARD" in
  stm32mp25) CORE=m33; APP=melt-pot-stm32mp25-app; IPC_BASE=0x20064000 ;;
  imx93) CORE=imx-rproc; APP=melt-pot-imx93-app; IPC_BASE=0x20480000 ;;
  *) echo "usage: sh run.sh <imx93|stm32mp25> [dist-dir]" >&2; exit 2 ;;
esac

LOG="/tmp/hil-$BOARD-$(date +%Y%m%d-%H%M%S).log"
die() { echo "FAIL: $*" >&2; exit 1; }

echo "== Stop the previous run"
command -v systemctl >/dev/null && systemctl stop consortium-app 2>/dev/null
killall "$APP" 2>/dev/null && sleep 1

RPROC=""
for r in /sys/class/remoteproc/remoteproc*; do
  [ "$(cat "$r/name" 2>/dev/null)" = "$CORE" ] && RPROC="$r"
done
[ -n "$RPROC" ] || die "no remoteproc named '$CORE' (is the tailored DTB booted?)"
[ "$(cat "$RPROC/state")" = offline ] || echo stop > "$RPROC/state"

# The CM33 never clears the IPC window: a descriptor left by the previous run
# would let it skip this run's handshake. The application zeroes the window
# again when it maps it.
echo "== Clear the stale IPC descriptor at $IPC_BASE"
if command -v devmem2 >/dev/null; then
  devmem2 "$IPC_BASE" w 0 >/dev/null
elif command -v devmem >/dev/null; then
  devmem "$IPC_BASE" 32 0
elif command -v memtool >/dev/null; then
  memtool -32 "$IPC_BASE=0" >/dev/null
else
  echo "   no devmem2/devmem/memtool: skipped (a stale descriptor can make M6 fail)"
fi

echo "== Load $DIST/firmware/cm33.elf on $(basename "$RPROC") ($CORE)"
[ -r "$DIST/firmware/cm33.elf" ] || die "$DIST/firmware/cm33.elf not found"
cp "$DIST/firmware/cm33.elf" "/lib/firmware/$FW_NAME" || die "copy to /lib/firmware failed"
echo "$FW_NAME" > "$RPROC/firmware"
mark="$(dmesg | wc -l)"
echo start > "$RPROC/state"
sleep 1
dmesg | tail -n +"$((mark + 1))" | grep -i -E "remoteproc|rproc"
[ "$(cat "$RPROC/state")" = running ] || die "remoteproc state is '$(cat "$RPROC/state")'"

echo "== Run $APP for ${DURATION}s (log: $LOG)"
[ -x "$DIST/app/$APP" ] || die "$DIST/app/$APP not found or not executable"
RUST_LOG="${RUST_LOG:-info}" NO_COLOR=1 timeout -s INT "$DURATION" "$DIST/app/$APP" 2>&1 | tee "$LOG"

echo
echo "== Stages"
# <id> <required?> <description>, in protocol order. A6 samples a doorbell
# that is consumed within microseconds, so they may legitimately be absent.
failed=0
while read -r id need what; do
  line="$(grep -E "HIL $id (PASS|FAIL)" "$LOG" | head -n 1)"
  case "$line" in
    *"HIL $id FAIL"*) verdict=FAIL; failed=1 ;;
    *"HIL $id PASS"*) verdict=PASS ;;
    *) if [ "$need" = yes ]; then verdict=MISSING; failed=1; else verdict="-"; fi ;;
  esac
  printf '  %-4s %-7s %s\n' "$id" "$verdict" "$what"
done <<'EOF'
A1 yes application started
A2 yes firmware loaded (remoteproc running)
M1 yes firmware launched, defmt ring up
A3 yes UIO windows mapped
A4 yes defmt shared memory initialized
M2 yes interrupt routing (vector table, NVIC)
M3 yes M-core waiting for connection (line armed)
A5 yes connection requested
M4 yes M-core sees the published descriptor
A6 no AP->M doorbell asserted (sampled)
M5 yes M-core interrupt fired
A7 yes M-core consumed the doorbell
M6 yes M-core connected and acked
A8 yes A-core interrupt fired
A9 yes A-core responded (handshake complete)
M7 yes M-core message sent
A10 yes message in the slot
A11 yes A-core message interrupt fired
A12 yes A-core message read
M8 yes M-core saw the message consumed
EOF

if grep -q "HIL .* FAIL\|HIL trace" "$LOG"; then
  failed=1
  echo
  echo "== Failures and traces"
  grep "HIL .* FAIL\|HIL trace\|HIL .*still\|HIL .*gap\|HIL .* has not" "$LOG"
fi
echo
if [ "$failed" = 0 ]; then echo "RESULT: PASS ($LOG)"; else echo "RESULT: FAIL ($LOG)"; fi
[ "$failed" = 0 ]
