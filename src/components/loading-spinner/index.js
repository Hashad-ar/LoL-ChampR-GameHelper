import s from './style.module.scss';

import React from 'react';
import cn from 'classnames';
import { Loader } from 'react-feather';

// colors: #1E54B7, #3D68EC
export default function LoadingSpinner({ className, size = 36, color = `#3D68EC` }) {
  return <Loader size={size} color={color} className={cn(s.loading, className)} />;
}
