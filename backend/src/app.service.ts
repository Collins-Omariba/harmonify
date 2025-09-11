import { Injectable } from '@nestjs/common';

@Injectable()
export class AppService {
  getPing(): { message: string } {
    return { message: 'Harmonify.' };
  }
}
