#[doc = "Register `MIR0_CMDREQ` reader"]
pub type R = crate::R<Mir0CmdreqSpec>;
#[doc = "Register `MIR0_CMDREQ` writer"]
pub type W = crate::W<Mir0CmdreqSpec>;
#[doc = "Field `MSGNUM` reader - Message Number"]
pub type MsgnumR = crate::FieldReader;
#[doc = "Field `MSGNUM` writer - Message Number"]
pub type MsgnumW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `BUSY` reader - Busy Flag"]
pub type BusyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - Message Number"]
    #[inline(always)]
    pub fn msgnum(&self) -> MsgnumR {
        MsgnumR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 15 - Busy Flag"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Message Number"]
    #[inline(always)]
    pub fn msgnum(&mut self) -> MsgnumW<'_, Mir0CmdreqSpec> {
        MsgnumW::new(self, 0)
    }
}
#[doc = "Interface Command Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mir0_cmdreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mir0_cmdreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mir0CmdreqSpec;
impl crate::RegisterSpec for Mir0CmdreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mir0_cmdreq::R`](R) reader structure"]
impl crate::Readable for Mir0CmdreqSpec {}
#[doc = "`write(|w| ..)` method takes [`mir0_cmdreq::W`](W) writer structure"]
impl crate::Writable for Mir0CmdreqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIR0_CMDREQ to value 0x01"]
impl crate::Resettable for Mir0CmdreqSpec {
    const RESET_VALUE: u32 = 0x01;
}
