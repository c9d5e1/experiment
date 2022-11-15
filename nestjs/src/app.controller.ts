import { Controller, Get } from '@nestjs/common';
import { AppService } from './app.service';

@Controller()
export class AppController {
  constructor(private readonly appService: AppService) {}

  @Get()
  getHello(): string {
    return 'Hello World!';
  }

  @Get('sleep')
  async getSleepHello(): Promise<string> {
    await this.sleep(5000);
    return 'Hello World!';
  }

  sleep(ms): Promise<any> {
    return new Promise((resolve) => {
      setTimeout(resolve, ms);
    });
  }
}
