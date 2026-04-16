//
// Copyright (c) 2026
//
// This file is part of the Renode peripheral models.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  SPDX-License-Identifier: Apache-2.0
//
// STMicroelectronics STM32MP25x Hardware Semaphore (HSEM) model.
// Reference: RM0457 Rev 5, Section 22.
//
// Implemented:
//  * 16 semaphores (HSEM_Rx / HSEM_RLRx) with 2-step lock, 1-step lock and
//    unlock semantics, including LOCKID/PROCID/SEC/PRIV/CFEN tracking.
//  * Per-processor (n = 1..=3) non-secure and secure interrupt enable / clear /
//    status / masked-status registers.
//  * Per-semaphore security (SECCFGR) and privilege (PRIVCFGR) attribute
//    configuration.
//  * Clear register (CR) + key register (KEYR) semantics, including the
//    "KEY must match" guard for mass unlocks.
//  * CID configuration registers (per-processor and per-group) plumbed enough
//    for most drivers - the CID filter itself is modelled permissively (all
//    accesses are considered to come from a matching CID) because Renode has
//    no global notion of compartment IDs; reads and writes of the config
//    registers still behave as described.
//  * Hardware info registers (HWCFGR1/2, VERR, IPIDR, SIDR) with fixed reset
//    values as per the reference manual.
//
// Deliberately simplified:
//  * The CID (compartment-ID) filter logic does not gate accesses, since there
//    is no emulated CID bus master in Renode. CFEN/CID fields are still
//    writable and readable for driver probing.
//  * Access-rights violations (secure-only / privileged-only writes) are not
//    enforced against the initiating CPU's privilege level - Renode's bus
//    transaction does not carry enough information in all configurations.
//    Drivers that only write from the intended context will behave correctly.
//
// Outputs:
//  * Three non-secure interrupt lines (IRQ[0..2]) - one per processor.
//  * Three secure interrupt lines     (IRQ[3..5]) - one per processor.
//

using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using Antmicro.Renode.Core;
using Antmicro.Renode.Core.Structure.Registers;
using Antmicro.Renode.Logging;

namespace Antmicro.Renode.Peripherals.Miscellaneous
{
    public class STM32MP25_HSEM : BasicDoubleWordPeripheral, IKnownSize, INumberedGPIOOutput
    {
        public STM32MP25_HSEM(IMachine machine) : base(machine)
        {
            locks = new SemaphoreState[SemaphoreCount];
            for(var i = 0; i < SemaphoreCount; i++)
            {
                locks[i] = new SemaphoreState();
            }

            // Per-processor interrupt state.
            nsIER   = new uint[ProcessorCount];
            nsISF   = new uint[ProcessorCount];
            secIER  = new uint[ProcessorCount];
            secISF  = new uint[ProcessorCount];
            cidCfg  = new uint[ProcessorCount];

            groupCidCfg = new uint[GroupCount];

            // IRQ outputs - indices 0..ProcessorCount-1 are non-secure,
            // indices ProcessorCount..2*ProcessorCount-1 are secure.
            var irqs = Enumerable.Range(0, ProcessorCount * 2)
                                 .ToDictionary(i => i, _ => (IGPIO)new GPIO());
            Connections = new ReadOnlyDictionary<int, IGPIO>(irqs);

            DefineRegisters();
        }

        public override void Reset()
        {
            base.Reset();
            for(var i = 0; i < SemaphoreCount; i++)
            {
                locks[i].Reset();
            }
            for(var n = 0; n < ProcessorCount; n++)
            {
                nsIER[n]  = 0;
                nsISF[n]  = 0;
                secIER[n] = 0;
                secISF[n] = 0;
                cidCfg[n] = 0;
            }
            for(var p = 0; p < GroupCount; p++)
            {
                groupCidCfg[p] = 0;
            }
            secCfg  = 0;
            privCfg = 0;
            keyReg  = 0;
            UpdateAllInterrupts();
        }

        public long Size => 0x400;

        public IReadOnlyDictionary<int, IGPIO> Connections { get; }

        // ----------------------------------------------------------------
        // Register map
        // ----------------------------------------------------------------
        private void DefineRegisters()
        {
            // HSEM_Rx (0x000 + 4*x), 2-step lock / read / unlock.
            for(var i = 0; i < SemaphoreCount; i++)
            {
                var idx = i;
                RegistersCollection.AddRegister(0x000 + 4 * idx,
                    new DoubleWordRegister(this)
                        .WithValueField(0, 8, name: "PROCID",
                            valueProviderCallback: _ => locks[idx].ProcId,
                            writeCallback: (_, v) =>
                            {
                                pendingRx[idx].ProcId = (byte)(v & 0xFF);
                                pendingRx[idx].ProcIdWritten = true;
                            })
                        .WithValueField(8, 4, name: "LOCKID",
                            valueProviderCallback: _ => locks[idx].LockId,
                            writeCallback: (_, v) =>
                            {
                                pendingRx[idx].LockId = (byte)(v & 0xF);
                                pendingRx[idx].LockIdWritten = true;
                            })
                        .WithFlag(12, name: "SEC",
                            valueProviderCallback: _ => locks[idx].Sec,
                            writeCallback: (_, v) =>
                            {
                                pendingRx[idx].Sec = v;
                                pendingRx[idx].SecWritten = true;
                            })
                        .WithFlag(13, name: "PRIV",
                            valueProviderCallback: _ => locks[idx].Priv,
                            writeCallback: (_, v) =>
                            {
                                pendingRx[idx].Priv = v;
                                pendingRx[idx].PrivWritten = true;
                            })
                        .WithReservedBits(14, 16)
                        .WithFlag(30, FieldMode.Read, name: "CFEN",
                            valueProviderCallback: _ => locks[idx].Cfen)
                        .WithFlag(31, name: "LOCK",
                            valueProviderCallback: _ => locks[idx].Locked,
                            writeCallback: (_, v) =>
                            {
                                pendingRx[idx].LockBit = v;
                                pendingRx[idx].LockBitWritten = true;
                            })
                        .WithWriteCallback((_, __) => CommitRxWrite(idx))
                );
            }

            // HSEM_RLRx (0x080 + 4*x), read-locks on read, all fields read-only.
            for(var i = 0; i < SemaphoreCount; i++)
            {
                var idx = i;
                RegistersCollection.AddRegister(0x080 + 4 * idx,
                    new DoubleWordRegister(this)
                        .WithValueField(0, 8, FieldMode.Read, name: "PROCID",
                            valueProviderCallback: _ => ReadLockAndGet(idx).ProcId)
                        .WithValueField(8, 4, FieldMode.Read, name: "LOCKID",
                            valueProviderCallback: _ => ReadLockAndGet(idx).LockId)
                        .WithFlag(12, FieldMode.Read, name: "SEC",
                            valueProviderCallback: _ => ReadLockAndGet(idx).Sec)
                        .WithFlag(13, FieldMode.Read, name: "PRIV",
                            valueProviderCallback: _ => ReadLockAndGet(idx).Priv)
                        .WithReservedBits(14, 16)
                        .WithFlag(30, FieldMode.Read, name: "CFEN",
                            valueProviderCallback: _ => ReadLockAndGet(idx).Cfen)
                        .WithFlag(31, FieldMode.Read, name: "LOCK",
                            valueProviderCallback: _ => ReadLockAndGet(idx).Locked)
                );
            }

            // Non-secure interrupt registers: CnIER / CnICR / CnISR / CnMISR
            // per processor n = 1..=3 at base 0x100 + 0x10 * (n-1).
            for(var n = 0; n < ProcessorCount; n++)
            {
                var proc = n;

                // CnIER at +0x00 - rw
                RegistersCollection.AddRegister(0x100 + 0x10 * proc,
                    new DoubleWordRegister(this)
                        .WithValueField(0, 16, name: "ISE",
                            valueProviderCallback: _ => nsIER[proc] & 0xFFFFu,
                            writeCallback: (_, v) => { nsIER[proc] = (uint)(v & 0xFFFF); UpdateNsInterrupt(proc); })
                        .WithReservedBits(16, 16)
                );

                // CnICR at +0x04 - write-1-to-clear of ISF bits
                RegistersCollection.AddRegister(0x104 + 0x10 * proc,
                    new DoubleWordRegister(this)
                        .WithValueField(0, 16, FieldMode.Write, name: "ISC",
                            writeCallback: (_, v) => { nsISF[proc] &= ~(uint)(v & 0xFFFF); UpdateNsInterrupt(proc); })
                        .WithReservedBits(16, 16)
                );

                // CnISR at +0x08 - read-only status before masking
                RegistersCollection.AddRegister(0x108 + 0x10 * proc,
                    new DoubleWordRegister(this)
                        .WithValueField(0, 16, FieldMode.Read, name: "ISF",
                            valueProviderCallback: _ => nsISF[proc] & 0xFFFFu)
                        .WithReservedBits(16, 16)
                );

                // CnMISR at +0x0C - read-only masked status
                RegistersCollection.AddRegister(0x10C + 0x10 * proc,
                    new DoubleWordRegister(this)
                        .WithValueField(0, 16, FieldMode.Read, name: "MISF",
                            valueProviderCallback: _ => (nsISF[proc] & nsIER[proc]) & 0xFFFFu)
                        .WithReservedBits(16, 16)
                );
            }

            // Secure interrupt registers: SCnIER / SCnICR / SCnISR / SCnMISR
            // per processor n = 1..=3 at base 0x180 + 0x10 * (n-1).
            for(var n = 0; n < ProcessorCount; n++)
            {
                var proc = n;

                RegistersCollection.AddRegister(0x180 + 0x10 * proc,
                    new DoubleWordRegister(this)
                        .WithValueField(0, 16, name: "SISE",
                            valueProviderCallback: _ => secIER[proc] & 0xFFFFu,
                            writeCallback: (_, v) => { secIER[proc] = (uint)(v & 0xFFFF); UpdateSecInterrupt(proc); })
                        .WithReservedBits(16, 16)
                );

                RegistersCollection.AddRegister(0x184 + 0x10 * proc,
                    new DoubleWordRegister(this)
                        .WithValueField(0, 16, FieldMode.Write, name: "SISC",
                            writeCallback: (_, v) => { secISF[proc] &= ~(uint)(v & 0xFFFF); UpdateSecInterrupt(proc); })
                        .WithReservedBits(16, 16)
                );

                RegistersCollection.AddRegister(0x188 + 0x10 * proc,
                    new DoubleWordRegister(this)
                        .WithValueField(0, 16, FieldMode.Read, name: "SISF",
                            valueProviderCallback: _ => secISF[proc] & 0xFFFFu)
                        .WithReservedBits(16, 16)
                );

                RegistersCollection.AddRegister(0x18C + 0x10 * proc,
                    new DoubleWordRegister(this)
                        .WithValueField(0, 16, FieldMode.Read, name: "SMISF",
                            valueProviderCallback: _ => (secISF[proc] & secIER[proc]) & 0xFFFFu)
                        .WithReservedBits(16, 16)
                );
            }

            // SECCFGR at 0x200 - per-semaphore security attribute.
            RegistersCollection.AddRegister(0x200,
                new DoubleWordRegister(this)
                    .WithValueField(0, 16, name: "SEC",
                        valueProviderCallback: _ => secCfg,
                        writeCallback: (_, v) => { secCfg = (uint)(v & 0xFFFF); })
                    .WithReservedBits(16, 16)
            );

            // PRIVCFGR at 0x210 - per-semaphore privilege attribute.
            RegistersCollection.AddRegister(0x210,
                new DoubleWordRegister(this)
                    .WithValueField(0, 16, name: "PRIV",
                        valueProviderCallback: _ => privCfg,
                        writeCallback: (_, v) => { privCfg = (uint)(v & 0xFFFF); })
                    .WithReservedBits(16, 16)
            );

            // CnCIDCFGR per processor at 0x220 + 4 * (n-1).
            for(var n = 0; n < ProcessorCount; n++)
            {
                var proc = n;
                RegistersCollection.AddRegister(0x220 + 4 * proc,
                    new DoubleWordRegister(this)
                        .WithFlag(0, name: "CFEN",
                            valueProviderCallback: _ => (cidCfg[proc] & 0x1) != 0,
                            writeCallback: (_, v) =>
                            {
                                if(v) cidCfg[proc] |= 0x1u;
                                else  cidCfg[proc] &= ~0x1u;
                            })
                        .WithReservedBits(1, 3)
                        .WithValueField(4, 3, name: "CID",
                            valueProviderCallback: _ => (cidCfg[proc] >> 4) & 0x7u,
                            writeCallback: (_, v) =>
                            {
                                cidCfg[proc] = (cidCfg[proc] & ~(0x7u << 4)) | (((uint)v & 0x7u) << 4);
                            })
                        .WithReservedBits(7, 25)
                );
            }

            // CR at 0x230 - mass clear-on-condition, key-protected.
            RegistersCollection.AddRegister(0x230,
                new DoubleWordRegister(this)
                    .WithFlag(0, FieldMode.Write, name: "CFEN",
                        writeCallback: (_, v) => { crPendingCfen = v; })
                    .WithReservedBits(1, 7)
                    .WithValueField(8, 4, FieldMode.Write, name: "LOCKID",
                        writeCallback: (_, v) => { crPendingLockId = (byte)(v & 0xF); })
                    .WithFlag(12, FieldMode.Write, name: "SEC",
                        writeCallback: (_, v) => { crPendingSec = v; })
                    .WithFlag(13, FieldMode.Write, name: "PRIV",
                        writeCallback: (_, v) => { crPendingPriv = v; })
                    .WithReservedBits(14, 2)
                    .WithValueField(16, 16, FieldMode.Write, name: "KEY",
                        writeCallback: (_, v) => { crPendingKey = (ushort)(v & 0xFFFF); })
                    .WithWriteCallback((_, __) => ExecuteMassClear())
            );

            // KEYR at 0x234 - stored clear-key value.
            RegistersCollection.AddRegister(0x234,
                new DoubleWordRegister(this)
                    .WithReservedBits(0, 16)
                    .WithValueField(16, 16, name: "KEY",
                        valueProviderCallback: _ => keyReg,
                        writeCallback: (_, v) => { keyReg = (ushort)(v & 0xFFFF); })
            );

            // GpCIDCFGR per group at 0x240 + 4 * p.
            for(var p = 0; p < GroupCount; p++)
            {
                var grp = p;
                RegistersCollection.AddRegister(0x240 + 4 * grp,
                    new DoubleWordRegister(this)
                        .WithFlag(0, name: "CFEN",
                            valueProviderCallback: _ => (groupCidCfg[grp] & 0x1) != 0,
                            writeCallback: (_, v) =>
                            {
                                if(v) groupCidCfg[grp] |= 0x1u;
                                else  groupCidCfg[grp] &= ~0x1u;
                            })
                        .WithReservedBits(1, 15)
                        .WithValueField(16, 3, name: "SEM_WLIST",
                            valueProviderCallback: _ => (groupCidCfg[grp] >> 16) & 0x7u,
                            writeCallback: (_, v) =>
                            {
                                groupCidCfg[grp] = (groupCidCfg[grp] & ~(0x7u << 16)) | (((uint)v & 0x7u) << 16);
                            })
                        .WithReservedBits(19, 13)
                );
            }

            // Read-only hardware info registers (fixed reset values per RM).
            RegistersCollection.AddRegister(0x3EC,
                new DoubleWordRegister(this, resetValue: 0x0000_0321)
                    .WithValueField(0, 32, FieldMode.Read, name: "HWCFGR2",
                        valueProviderCallback: _ => 0x0000_0321u)
            );
            RegistersCollection.AddRegister(0x3F0,
                new DoubleWordRegister(this, resetValue: 0x0007_3310)
                    .WithValueField(0, 32, FieldMode.Read, name: "HWCFGR1",
                        valueProviderCallback: _ => 0x0007_3310u)
            );
            RegistersCollection.AddRegister(0x3F4,
                new DoubleWordRegister(this, resetValue: 0x0000_0020)
                    .WithValueField(0, 32, FieldMode.Read, name: "VERR",
                        valueProviderCallback: _ => 0x0000_0020u)
            );
            RegistersCollection.AddRegister(0x3F8,
                new DoubleWordRegister(this, resetValue: 0x0010_0072)
                    .WithValueField(0, 32, FieldMode.Read, name: "IPIDR",
                        valueProviderCallback: _ => 0x0010_0072u)
            );
            RegistersCollection.AddRegister(0x3FC,
                new DoubleWordRegister(this, resetValue: unchecked((uint)0xA3C5_DD01))
                    .WithValueField(0, 32, FieldMode.Read, name: "SIDR",
                        valueProviderCallback: _ => 0xA3C5_DD01u)
            );
        }

        // ----------------------------------------------------------------
        // HSEM_Rx write commit.
        //
        // A single 32-bit register write fires each field's writeCallback in
        // turn (which populates `pendingRx[idx]`), then finally the
        // WithWriteCallback aggregate which calls CommitRxWrite. This way we
        // observe the full atomic set of field values that software wrote.
        // ----------------------------------------------------------------

        private void CommitRxWrite(int idx)
        {
            var staged = pendingRx[idx];
            pendingRx[idx] = default; // clear for next write

            var sem = locks[idx];

            // Determine the "attempting" identity - fall back to the currently
            // locked identity if a field wasn't written in this transaction.
            var reqLockId = staged.LockIdWritten ? staged.LockId : sem.LockId;
            var reqProcId = staged.ProcIdWritten ? staged.ProcId : sem.ProcId;
            var reqSec    = staged.SecWritten    ? staged.Sec    : sem.Sec;
            var reqPriv   = staged.PrivWritten   ? staged.Priv   : sem.Priv;
            var lockBit   = staged.LockBitWritten ? staged.LockBit : sem.Locked;

            if(lockBit)
            {
                // 2-step lock attempt.
                if(!sem.Locked)
                {
                    sem.Locked = true;
                    sem.LockId = reqLockId;
                    sem.ProcId = reqProcId;
                    sem.Sec    = reqSec;
                    sem.Priv   = reqPriv;
                    sem.Cfen   = false;
                    this.DebugLog("HSEM: R{0} LOCK acquired (LOCKID={1}, PROCID=0x{2:X})", idx, reqLockId, reqProcId);
                }
                else
                {
                    // Already locked - write ignored, LOCK reads as 1 on next read.
                    this.DebugLog("HSEM: R{0} LOCK attempt while already locked - ignored", idx);
                }
            }
            else
            {
                // Unlock attempt - identity must match the locking identity.
                if(sem.Locked && sem.LockId == reqLockId && sem.ProcId == reqProcId
                              && sem.Sec == reqSec && sem.Priv == reqPriv)
                {
                    ReleaseAndNotify(idx, sem);
                }
                else if(sem.Locked)
                {
                    this.WarningLog("HSEM: R{0} unlock ID mismatch - ignoring (have LOCKID={1} PROCID=0x{2:X}, got LOCKID={3} PROCID=0x{4:X})",
                        idx, sem.LockId, sem.ProcId, reqLockId, reqProcId);
                }
            }
        }

        // ----------------------------------------------------------------
        // RLR read: 1-step lock (RM0457 §22.3.3).
        // Reading an unlocked semaphore locks it in the name of the reader.
        // Because we have no per-access CID/PROCID context, we record the
        // semaphore as locked with PROCID=0 (per RM) and LOCKID taken from
        // the last HSEM_Rx write's LOCKID - or 0 if never written.
        //
        // The returned value reflects the state *after* the lock attempt.
        // ----------------------------------------------------------------
        private SemaphoreState ReadLockAndGet(int idx)
        {
            var sem = locks[idx];
            if(!sem.Locked)
            {
                sem.Locked = true;
                sem.ProcId = 0; // RM0457: 1-step lock always has PROCID = 0
                // LOCKID is sourced from the master's ID bus; we cannot observe
                // it in Renode, so leave it at its previous value (0 at reset).
                sem.Sec  = false;
                sem.Priv = false;
                sem.Cfen = false;
                this.DebugLog("HSEM: RLR{0} 1-step LOCK acquired (LOCKID={1})", idx, sem.LockId);
            }
            return sem;
        }

        // ----------------------------------------------------------------
        // Release a single semaphore: mark free, raise ISF bits in every
        // processor IER that has the corresponding bit enabled (based on
        // SECCFGR routing).
        // ----------------------------------------------------------------
        private void ReleaseAndNotify(int idx, SemaphoreState sem)
        {
            sem.Locked = false;
            sem.LockId = 0;
            sem.ProcId = 0;
            sem.Sec    = false;
            sem.Priv   = false;
            sem.Cfen   = false;

            var bit = 1u << idx;
            var isSecure = (secCfg & bit) != 0;

            for(var n = 0; n < ProcessorCount; n++)
            {
                if(isSecure)
                {
                    secISF[n] |= bit;
                }
                else
                {
                    nsISF[n] |= bit;
                }
            }

            this.DebugLog("HSEM: R{0} released - notifying {1} processors", idx, ProcessorCount);
            UpdateAllInterrupts();
        }

        // ----------------------------------------------------------------
        // CR mass clear (RM0457 §22.5.14).
        // Releases all semaphores whose (LOCKID, SEC, PRIV, CFEN) match the
        // fields written to CR - but only if the KEY field also matches
        // HSEM_KEYR.KEY.
        // ----------------------------------------------------------------
        private void ExecuteMassClear()
        {
            if(crPendingKey != keyReg)
            {
                this.WarningLog("HSEM: CR write with mismatched KEY (0x{0:X4} vs stored 0x{1:X4}) - no semaphores cleared",
                    crPendingKey, keyReg);
                return;
            }

            var released = 0;
            for(var i = 0; i < SemaphoreCount; i++)
            {
                var sem = locks[i];
                if(!sem.Locked)
                {
                    continue;
                }
                if(sem.LockId == crPendingLockId
                   && sem.Sec  == crPendingSec
                   && sem.Priv == crPendingPriv
                   && sem.Cfen == crPendingCfen)
                {
                    ReleaseAndNotify(i, sem);
                    released++;
                }
            }
            this.DebugLog("HSEM: CR mass-clear released {0} semaphore(s) matching LOCKID={1} SEC={2} PRIV={3} CFEN={4}",
                released, crPendingLockId, crPendingSec, crPendingPriv, crPendingCfen);
        }

        // ----------------------------------------------------------------
        // Interrupt propagation.
        // ----------------------------------------------------------------
        private void UpdateAllInterrupts()
        {
            for(var n = 0; n < ProcessorCount; n++)
            {
                UpdateNsInterrupt(n);
                UpdateSecInterrupt(n);
            }
        }

        private void UpdateNsInterrupt(int proc)
        {
            var pending = (nsISF[proc] & nsIER[proc]) != 0;
            Connections[proc].Set(pending);
        }

        private void UpdateSecInterrupt(int proc)
        {
            var pending = (secISF[proc] & secIER[proc]) != 0;
            Connections[ProcessorCount + proc].Set(pending);
        }

        // ----------------------------------------------------------------
        // State
        // ----------------------------------------------------------------
        private const int SemaphoreCount = 16;
        private const int ProcessorCount = 3;
        private const int GroupCount     = 4;

        private readonly SemaphoreState[] locks;

        private readonly uint[] nsIER;
        private readonly uint[] nsISF;
        private readonly uint[] secIER;
        private readonly uint[] secISF;
        private readonly uint[] cidCfg;
        private readonly uint[] groupCidCfg;

        private uint  secCfg;
        private uint  privCfg;
        private ushort keyReg;

        // CR staging for atomic commit.
        private bool   crPendingCfen;
        private byte   crPendingLockId;
        private bool   crPendingSec;
        private bool   crPendingPriv;
        private ushort crPendingKey;

        // Per-Rx staging for atomic commit.
        private readonly RxWriteStaging[] pendingRx = new RxWriteStaging[SemaphoreCount];

        private struct RxWriteStaging
        {
            public byte LockId;         public bool LockIdWritten;
            public byte ProcId;         public bool ProcIdWritten;
            public bool Sec;            public bool SecWritten;
            public bool Priv;           public bool PrivWritten;
            public bool LockBit;        public bool LockBitWritten;
        }

        private class SemaphoreState
        {
            public bool Locked;
            public byte LockId;
            public byte ProcId;
            public bool Sec;
            public bool Priv;
            public bool Cfen;

            public void Reset()
            {
                Locked = false;
                LockId = 0;
                ProcId = 0;
                Sec    = false;
                Priv   = false;
                Cfen   = false;
            }
        }
    }
}
