/*
 * cog-config.h
 * Copyright (C) 2021 Igalia S.L.
 * Copyright (C) 2017-2018 Adrian Perez <aperez@igalia.com>
 *
 * Distributed under terms of the MIT license.
 */

#ifndef COG_CONFIG_H_IN
#define COG_CONFIG_H_IN

#define COG_MODULEDIR "/usr/lib/aarch64-linux-gnu/cog/modules"
#define COG_VERSION_MAJOR 0
#define COG_VERSION_MINOR 19
#define COG_VERSION_PATCH 1
#define COG_VERSION_STRING "0.19.1"
#define COG_VERSION_EXTRA "+git-b19d72a"
#define COG_DEFAULT_APPID "com.igalia.Cog"
#define COG_DEFAULT_HOME_URI ""
#define COG_HAVE_MEM_PRESSURE 1
#define COG_ENABLE_GAMEPAD_MANETTE 0

/* FIXME: Perhaps make this a meson define instead. */
#define COG_DEFAULT_APPNAME "Cog"

#endif /* !COG_CONFIG_H_IN */
