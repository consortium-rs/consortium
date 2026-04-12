# Copyright 2026 Ethan Wu
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# SPDX-License-Identifier: Apache-2.0

import xml.etree.ElementTree as ET
import sys
import re

def natural_sort_key(name):
    """Convert name to a key for natural sorting (handles numeric parts)."""
    parts = []
    for part in re.split(r'(\d+)', name):
        parts.append(int(part) if part.isdigit() else part)
    return parts

def parse_signal_name(signal_name):
    """
    Parse a signal name like 'ADC1_INN15' into:
    name: adc1, pin: inn15
    """
    parts = signal_name.split('_', 1)
    if len(parts) == 2:
        return parts[0].lower(), parts[1].lower()
    else:
        return signal_name.lower(), None

def extract_pins(xml_file):
    tree = ET.parse(xml_file)
    root = tree.getroot()

    # Handle namespace if present
    ns = ''
    if root.tag.startswith('{'):
        ns = root.tag.split('}')[0] + '}'

    pins = []

    for pin_elem in root.findall(f'{ns}Pin'):
        pin_name = pin_elem.get('Name')
        position = pin_elem.get('Position')
        pin_type = pin_elem.get('Type')

        # Skip power, reset, boot, and mono pins if desired
        # (comment out the next 2 lines to include all)
        if pin_type not in ('I/O',):
            pass  # still include them, just no signals

        signals = []
        for sig_elem in pin_elem.findall(f'{ns}Signal'):
            sig_name = sig_elem.get('Name')
            if sig_name and sig_name != 'GPIO':
                name, pin = parse_signal_name(sig_name)
                signals.append((name, pin))

        pins.append({
            'name': pin_name,
            'position': position,
            'type': pin_type,
            'signals': signals,
        })

    return pins

def render_yaml(pins):
    lines = [i for i in '''# ------------------------------------------------------
# Derived from STM32CubeMX IO configuration
# Only for use with STMicroelectronics STM32 MPU devices
# License: ST SLA0044 / SLA0048
# Purpose: IO mapping, pin configuration validation
# ------------------------------------------------------
'''.split('\n')]
    for pin in pins:
        lines.append(f"{pin['name']}:")
        lines.append(f"  position: {pin['position']}")
        lines.append(f"  type: {pin['type']}")
        if pin['signals']:
            lines.append(f"  mux:")
            for name, pin_part in pin['signals']:
                if pin_part:
                    lines.append(f"    - name: {name.lower()}")
                    lines.append(f"      pin: {pin_part.lower()}")
                else:
                    lines.append(f"    - name: {name}")
        lines.append("")
    return '\n'.join(lines)

if __name__ == '__main__':
    xml_file = sys.argv[1] if len(sys.argv) > 1 else 'mcu.xml'
    pins = extract_pins(xml_file)

    # Filter to only I/O pins for cleaner output (remove filter for all pins)
    io_pins = [p for p in pins if p['type'] == 'I/O']

    # Sort pins by name (natural sort)
    io_pins.sort(key=lambda p: natural_sort_key(p['name']))

    output = render_yaml(io_pins)
    print(output)