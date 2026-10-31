<?php

declare(strict_types=1);

namespace Switon\Ammonia;

/**
 * {@see Builder::clean()} failed because the setter combination is one ammonia rejects.
 *
 * For example, `class` is a generic attribute and {@see Builder::addAllowedClasses()} is also set, or `rel` is allowed without {@see Builder::linkRel()} set to `null`.
 *
 * IDE stub. The extension provides this class at runtime. Do not include this file.
 */
class ConfigurationException extends Exception
{
}
