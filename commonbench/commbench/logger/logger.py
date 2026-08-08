"""ColorFormatter and FixedLineHandler extensions for logging.

This module adds coloring and fixed line handling to python's
builtin logger module.
"""

import sys
import time
import logging
from enum import Enum
from collections import deque


class Color(Enum):
    """An Enum class for different terminal colors."""

    bold_red = "\033[41m"
    grey     = "\033[38m"
    cyan     = "\033[36m"
    blue     = "\033[34m"
    yellow   = "\033[33m"
    green    = "\033[32m"
    red      = "\033[31m"
    reset    = "\033[0m"


class ColorFormatter(logging.Formatter):
    """Add different colors for different segments of log messages."""

    LEVEL_COLORS = {
        logging.DEBUG:    Color.cyan.value,
        logging.INFO:     Color.green.value,
        logging.WARNING:  Color.yellow.value,
        logging.ERROR:    Color.red.value,
        logging.CRITICAL: Color.bold_red.value
    }
    FIELD_COLORS = {
        "asctime":   Color.blue.value,
        "name":      Color.green.value,
        "filename":  Color.cyan.value,
        "lineno":    Color.yellow.value,
        "levelname": Color.red.value,
        "message":   Color.grey.value,
    }
    FIELD_WIDTHS = {
        "asctime":   24,
        "name":      34,
        "file_line": 44,  # [filename:lineno]
        "levelname": 18,
    }

    def __init__(self, fmt=None, datefmt=None):
        if fmt and "%(lineno)d" in fmt:
            fmt = fmt.replace("%(lineno)d", "%(lineno)s")
        super().__init__(fmt, datefmt)

    def formatTime(self, record, datefmt=None):
        asctime = super().formatTime(record, datefmt)
        colored_time = f"{self.FIELD_COLORS['asctime']}{asctime}{Color.reset.value}"
        return f"{colored_time}: ".rjust(self.FIELD_WIDTHS["asctime"])

    def format(self, record):
        original_values = {
            'name':      record.name,
            'filename':  record.filename,
            'lineno':    record.lineno,
            'levelname': record.levelname
        }

        record.name = (f"{self.FIELD_COLORS['name']}{record.name}{Color.reset.value}: ".rjust(self.FIELD_WIDTHS["name"]))

        colored_file_line = (f"[{self.FIELD_COLORS['filename']}{record.filename}{Color.reset.value}:{self.FIELD_COLORS['lineno']}{record.lineno}{Color.reset.value}]: ")
        padded_file_line = colored_file_line.rjust(self.FIELD_WIDTHS["file_line"])
        record.filename = f"{padded_file_line}"

        record.levelname = (f"{self.LEVEL_COLORS.get(record.levelno, Color.reset.value)}{original_values['levelname']}{Color.reset.value}: ".rjust(self.FIELD_WIDTHS["levelname"]))

        formatted_message = super().format(record)
        formatted_message = formatted_message.replace(str(record.msg), f"{self.FIELD_COLORS['message']}{record.msg}{Color.reset.value}")

        for key, value in original_values.items():
            setattr(record, key, value)

        return formatted_message


class FixedLineHandler(logging.Handler):
    """Add fixed line handling for logging module."""

    def __init__(self, max_lines=10, delay=0.2):
        super().__init__()
        self.max_lines = max_lines
        self.delay = delay
        self.log_lines = deque(maxlen=max_lines if max_lines > 0 else None)
        if max_lines > 0:
            # Initial empty lines for creating display area
            print('\n' * (max_lines - 1))

    def emit(self, record):
        msg = self.format(record)
        self.log_lines.append(msg)

        if self.max_lines > 0:
            # Move cursor up to start of display area
            sys.stdout.write(f'\033[{self.max_lines}A')

            # Print all lines, padding with empty lines if needed
            current_lines = list(self.log_lines)
            while len(current_lines) < self.max_lines:
                current_lines.append('')

            print('\n'.join(current_lines))
        else:
            print(msg)

        sys.stdout.flush()
        time.sleep(self.delay)
