use crate::dram_lib::{read_phy, read_pi, write_phy, write_pi};

pub fn init(base_addr: usize, ddr_num: usize) {
    let tmp = read_phy(base_addr, 89);
    write_phy(base_addr, 89, tmp & 0xffffff00 | 0x00000051); //0x51->0x61

    //disable RDLVL VREF
    let tmp = read_phy(base_addr, 78);
    write_phy(base_addr, 78, tmp & 0xfffffcff);

    let tmp = read_phy(base_addr, 345);
    write_phy(base_addr, 345, tmp & 0xffffff00 | 0x00000051); //0x51->0x61

    //disable RDLVL VREF
    let tmp = read_phy(base_addr, 334);
    write_phy(base_addr, 334, tmp & 0xfffffcff);

    let tmp = read_phy(base_addr, 601);
    write_phy(base_addr, 601, tmp & 0xffffff00 | 0x00000051); //0x51->0x61

    //disable RDLVL VREF
    let tmp = read_phy(base_addr, 590);
    write_phy(base_addr, 590, tmp & 0xfffffcff);

    let tmp = read_phy(base_addr, 857);
    write_phy(base_addr, 857, tmp & 0xffffff00 | 0x00000051); //0x51->0x61

    //disable RDLVL VREF
    let tmp = read_phy(base_addr, 846);
    write_phy(base_addr, 846, tmp & 0xfffffcff);

    //turn off multicast
    let tmp = read_phy(base_addr, 1793);
    write_phy(base_addr, 1793, tmp & 0xfffffeff);

    //set to freq copy 0
    let tmp = read_phy(base_addr, 1793);
    write_phy(base_addr, 1793, tmp & 0xfffcffff);

    //data slice registers
    let tmp = read_phy(base_addr, 125);
    write_phy(base_addr, 125, tmp & 0xfff0ffff | 0x00010000);

    let tmp = read_phy(base_addr, 102);
    write_phy(base_addr, 102, tmp & 0xfffffffc | 0x00000001);

    let tmp = read_phy(base_addr, 105);
    write_phy(base_addr, 105, tmp & 0xffffffe0 | 0x00000001);

    let tmp = read_phy(base_addr, 92);
    write_phy(base_addr, 92, tmp & 0xfffffffe | 0x00000001);

    let tmp = read_phy(base_addr, 94);
    write_phy(base_addr, 94, tmp & 0xffffe0ff | 0x00000200);

    let tmp = read_phy(base_addr, 96);
    write_phy(base_addr, 96, tmp & 0xfffff0ff | 0x00000400);

    let tmp = read_phy(base_addr, 89);
    write_phy(base_addr, 89, tmp & 0xffffff00 | 0x00000051);

    let tmp = read_phy(base_addr, 381);
    write_phy(base_addr, 381, tmp & 0xfff0ffff | 0x00010000);

    let tmp = read_phy(base_addr, 358);
    write_phy(base_addr, 358, tmp & 0xfffffffc | 0x00000001);

    let tmp = read_phy(base_addr, 361);
    write_phy(base_addr, 361, tmp & 0xffffffe0 | 0x00000001);

    let tmp = read_phy(base_addr, 348);
    write_phy(base_addr, 348, tmp & 0xfffffffe | 0x00000001);

    let tmp = read_phy(base_addr, 350);
    write_phy(base_addr, 350, tmp & 0xffffe0ff | 0x00000200);

    let tmp = read_phy(base_addr, 352);
    write_phy(base_addr, 352, tmp & 0xfffff0ff | 0x00000400);

    let tmp = read_phy(base_addr, 345);
    write_phy(base_addr, 345, tmp & 0xffffff00 | 0x00000051);

    let tmp = read_phy(base_addr, 637);
    write_phy(base_addr, 637, tmp & 0xfff0ffff | 0x00010000);

    let tmp = read_phy(base_addr, 614);
    write_phy(base_addr, 614, tmp & 0xfffffffc | 0x00000001);

    let tmp = read_phy(base_addr, 617);
    write_phy(base_addr, 617, tmp & 0xffffffe0 | 0x00000001);

    let tmp = read_phy(base_addr, 604);
    write_phy(base_addr, 604, tmp & 0xfffffffe | 0x00000001);

    let tmp = read_phy(base_addr, 606);
    write_phy(base_addr, 606, tmp & 0xffffe0ff | 0x00000200);

    let tmp = read_phy(base_addr, 608);
    write_phy(base_addr, 608, tmp & 0xfffff0ff | 0x00000400);

    let tmp = read_phy(base_addr, 601);
    write_phy(base_addr, 601, tmp & 0xffffff00 | 0x00000051);

    let tmp = read_phy(base_addr, 893);
    write_phy(base_addr, 893, tmp & 0xfff0ffff | 0x00010000);

    let tmp = read_phy(base_addr, 870);
    write_phy(base_addr, 870, tmp & 0xfffffffc | 0x00000001);

    let tmp = read_phy(base_addr, 873);
    write_phy(base_addr, 873, tmp & 0xffffffe0 | 0x00000001);

    let tmp = read_phy(base_addr, 860);
    write_phy(base_addr, 860, tmp & 0xfffffffe | 0x00000001);

    let tmp = read_phy(base_addr, 862);
    write_phy(base_addr, 862, tmp & 0xffffe0ff | 0x00000200);

    let tmp = read_phy(base_addr, 864);
    write_phy(base_addr, 864, tmp & 0xfffff0ff | 0x00000400);

    let tmp = read_phy(base_addr, 857);
    write_phy(base_addr, 857, tmp & 0xffffff00 | 0x00000051);

    //phy level registers
    let tmp = read_phy(base_addr, 1895);
    write_phy(base_addr, 1895, tmp & 0xffffe000 | 0x00001342);

    /* This is also commented out in the original implementation. */
    // for memory clock<=400M
    //read_phy(base_addr, 16'd16'd1895, tmp);
    //write_phy(base_addr, 16'd16'd1895, tmp&32'hfffe_ffff|32'h0001_0000);

    let tmp = read_phy(base_addr, 1835);
    write_phy(base_addr, 1835, tmp & 0xfffff0ff | 0x00000200);

    //turn on multicast
    let tmp = read_phy(base_addr, 1793);
    write_phy(base_addr, 1793, tmp & 0xfffffeff | 0x00000100);

    /* PI config */

    let tmp = read_pi(base_addr, 62);
    write_pi(base_addr, 62, tmp & 0xfffffeff);

    let tmp = read_pi(base_addr, 66);
    write_pi(base_addr, 66, tmp & 0xfffffeff);

    let tmp = read_pi(base_addr, 166);
    write_pi(base_addr, 166, tmp & 0xffffff80 | 0x00000001);

    let tmp = read_pi(base_addr, 62);
    write_pi(base_addr, 62, tmp & 0xfff0ffff | 0x00010000);

    let tmp = read_pi(base_addr, 62);
    write_pi(base_addr, 62, tmp & 0xf0ffffff | 0x01000000);

    let tmp = read_pi(base_addr, 166);
    write_pi(base_addr, 166, tmp & 0xffff80ff | 0x00000100);

    let tmp = read_pi(base_addr, 179);
    write_pi(base_addr, 179, tmp & 0xff80ffff | 0x00010000);

    let tmp = read_pi(base_addr, 67);
    write_pi(base_addr, 67, tmp & 0xffe0ffff | 0x00010000);

    let tmp = read_pi(base_addr, 67);
    write_pi(base_addr, 67, tmp & 0xe0ffffff | 0x01000000);

    let tmp = read_pi(base_addr, 179);
    write_pi(base_addr, 179, tmp & 0x80ffffff | 0x01000000);

    let tmp = read_pi(base_addr, 166);
    write_pi(base_addr, 166, tmp & 0xff80ffff | 0x00010000);

    let tmp = read_pi(base_addr, 62);
    write_pi(base_addr, 62, tmp & 0xfff0ffff | 0x00010000);

    let tmp = read_pi(base_addr, 62);
    write_pi(base_addr, 62, tmp & 0xf0ffffff | 0x01000000);

    let tmp = read_pi(base_addr, 166);
    write_pi(base_addr, 166, tmp & 0x80ffffff | 0x01000000);

    let tmp = read_pi(base_addr, 182);
    write_pi(base_addr, 182, tmp & 0xff80ffff | 0x00010000);

    let tmp = read_pi(base_addr, 67);
    write_pi(base_addr, 67, tmp & 0xffe0ffff | 0x00010000);

    let tmp = read_pi(base_addr, 67);
    write_pi(base_addr, 67, tmp & 0xe0ffffff | 0x01000000);

    let tmp = read_pi(base_addr, 182);
    write_pi(base_addr, 182, tmp & 0x80ffffff | 0x01000000);

    let tmp = read_pi(base_addr, 167);
    write_pi(base_addr, 167, tmp & 0xffffff80 | 0x00000001);

    let tmp = read_pi(base_addr, 62);
    write_pi(base_addr, 62, tmp & 0xfff0ffff | 0x00010000);

    let tmp = read_pi(base_addr, 62);
    write_pi(base_addr, 62, tmp & 0xf0ffffff | 0x01000000);

    let tmp = read_pi(base_addr, 167);
    write_pi(base_addr, 167, tmp & 0xffff80ff | 0x00000100);

    let tmp = read_pi(base_addr, 185);
    write_pi(base_addr, 185, tmp & 0xff80ffff | 0x00010000);

    let tmp = read_pi(base_addr, 67);
    write_pi(base_addr, 67, tmp & 0xffe0ffff | 0x00010000);

    let tmp = read_pi(base_addr, 67);
    write_pi(base_addr, 67, tmp & 0xe0ffffff | 0x01000000);

    let tmp = read_pi(base_addr, 185);
    write_pi(base_addr, 185, tmp & 0x80ffffff | 0x01000000);

    let tmp = read_pi(base_addr, 10);
    write_pi(base_addr, 10, tmp & 0xffffffe0 | 0x00000002);

    let tmp = read_pi(base_addr, 0);
    write_pi(base_addr, 0, tmp & 0xfffffffe | 0x00000001);

    /* The following is also commented out in the original implementation */
    //Reduce time for IO pad calibration TODO
    //let tmp = read(base_addr, 4096 +1860);
    //write(base_addr, 4096 +1860, tmp&0x80ffffff|0x01000000);

    //PI_CS_MAP: 0xf->0x3
    //read_pi(base_addr, 16'd16'd11, tmp);
    //if(tmp != 32'h0300080f) $stop;
    //write_pi(base_addr, 16'd16'd11, 32'h03000803);

    //set CS0 MR13.VRCG=1
    let tmp = read_pi(base_addr, 247);
    write_pi(base_addr, 247, tmp | 0x00000008);

    //set CS1 MR13.VRCG=1
    let tmp = read_pi(base_addr, 249);
    write_pi(base_addr, 249, tmp | 0x00000800);

    //set CS2 MR13.VRCG=1
    let tmp = read_pi(base_addr, 252);
    write_pi(base_addr, 252, tmp | 0x00000008);

    //set CS3 MR13.VRCG=1
    let tmp = read_pi(base_addr, 254);
    write_pi(base_addr, 254, tmp | 0x00000800);

    //PI_MR11_DATA_F1_X
    let tmp = read_pi(base_addr, 281);
    write_pi(base_addr, 281, tmp | 0x36000000);
    let tmp = read_pi(base_addr, 305);
    write_pi(base_addr, 305, tmp | 0x36000000);
    let tmp = read_pi(base_addr, 329);
    write_pi(base_addr, 329, tmp | 0x36000000);
    let tmp = read_pi(base_addr, 353);
    write_pi(base_addr, 353, tmp | 0x36000000);

    //PI_MR11_DATA_F2_X
    let tmp = read_pi(base_addr, 289);
    write_pi(base_addr, 289, tmp | 0x36000000);
    let tmp = read_pi(base_addr, 313);
    write_pi(base_addr, 313, tmp | 0x36000000);
    let tmp = read_pi(base_addr, 337);
    write_pi(base_addr, 337, tmp | 0x36000000);
    let tmp = read_pi(base_addr, 361);
    write_pi(base_addr, 361, tmp | 0x36000000);

    if true {
        //PI_MR22_DATA_F1_X
        let tmp = read_pi(base_addr, 282);
        write_pi(base_addr, 282, tmp | 0x001e0000);
        let tmp = read_pi(base_addr, 306);
        write_pi(base_addr, 306, tmp | 0x001e0000);
        let tmp = read_pi(base_addr, 330);
        write_pi(base_addr, 330, tmp | 0x001e0000);
        let tmp = read_pi(base_addr, 354);
        write_pi(base_addr, 354, tmp | 0x001e0000);

        //PI_MR22_DATA_F2_X
        let tmp = read_pi(base_addr, 290);
        write_pi(base_addr, 290, tmp | 0x001e0000);
        let tmp = read_pi(base_addr, 314);
        write_pi(base_addr, 314, tmp | 0x001e0000);
        let tmp = read_pi(base_addr, 338);
        write_pi(base_addr, 338, tmp | 0x001e0000);
        let tmp = read_pi(base_addr, 362);
        write_pi(base_addr, 362, tmp | 0x001e0000);
    }

    /* ---------------------------------------------------------------- */
    /* commented out part found in original implementation omitted here */
    /* ---------------------------------------------------------------- */

    //PHY_RPTR_UPDATE_x: bit[11:8]+=3
    let tmp = read_phy(base_addr, 96);
    write_phy(base_addr, 96, tmp + 0x0300);

    let tmp = read_phy(base_addr, 352);
    write_phy(base_addr, 352, tmp + 0x0300);

    let tmp = read_phy(base_addr, 608);
    write_phy(base_addr, 608, tmp + 0x0300);

    let tmp = read_phy(base_addr, 864);
    write_phy(base_addr, 864, tmp + 0x0300);

    /* ------------ */
    /* G_SPEED_3200 */
    //PHY_WRLVL_DLY_STEP_X: 8'hC -> 8'h18
    let tmp = read_phy(base_addr, 96);
    write_phy(base_addr, 96, tmp & 0xff00ffff | 0x00180000);

    let tmp = read_phy(base_addr, 352);
    write_phy(base_addr, 352, tmp & 0xff00ffff | 0x00180000);

    let tmp = read_phy(base_addr, 608);
    write_phy(base_addr, 608, tmp & 0xff00ffff | 0x00180000);

    let tmp = read_phy(base_addr, 864);
    write_phy(base_addr, 864, tmp & 0xff00ffff | 0x00180000);
    /* ------------ */
    // cases for other speeds found in original omitted here

    //PHY_WDQLVL_CLK_JITTER_TOLERANCE_X: 8'h20 -> 8'h40
    let tmp = read_phy(base_addr, 33);
    write_phy(base_addr, 33, tmp & 0xffffff00 | 0x0040);

    let tmp = read_phy(base_addr, 289);
    write_phy(base_addr, 289, tmp & 0xffffff00 | 0x0040);

    let tmp = read_phy(base_addr, 545);
    write_phy(base_addr, 545, tmp & 0xffffff00 | 0x0040);

    let tmp = read_phy(base_addr, 801);
    write_phy(base_addr, 801, tmp & 0xffffff00 | 0x0040);

    /* ---------------------------------------------------------------- */
    /* commented out part found in original implementation omitted here */
    /* ---------------------------------------------------------------- */

    let tmp = read_phy(base_addr, 1038);
    write_phy(base_addr, 1038, tmp & 0xfcffffff | 0x02000000);

    let tmp = read_phy(base_addr, 1294);
    write_phy(base_addr, 1294, tmp & 0xfcffffff | 0x02000000);

    let tmp = read_phy(base_addr, 1550);
    write_phy(base_addr, 1550, tmp & 0xfcffffff | 0x02000000);

    if true {
        //0807
        //PHY_PAD_DSLICE_IO_CFG_x:0->7
        let tmp = read_phy(base_addr, 83);
        write_phy(base_addr, 83, tmp & 0xffc0ffff | 0x70000);

        let tmp = read_phy(base_addr, 339);
        write_phy(base_addr, 339, tmp & 0xffc0ffff | 0x70000);

        let tmp = read_phy(base_addr, 595);
        write_phy(base_addr, 595, tmp & 0xffc0ffff | 0x70000);

        let tmp = read_phy(base_addr, 851);
        write_phy(base_addr, 851, tmp & 0xffc0ffff | 0x70000);

        //PHY_PAD_ADR_IO_CFG_x:0->7
        let tmp = read_phy(base_addr, 1062);
        write_phy(base_addr, 1062, tmp & 0xf800ffff | 0x70000);

        let tmp = read_phy(base_addr, 1318);
        write_phy(base_addr, 1318, tmp & 0xf800ffff | 0x70000);

        let tmp = read_phy(base_addr, 1574);
        write_phy(base_addr, 1574, tmp & 0xf800ffff | 0x70000);

        //PHY_PAD_CAL_IO_CFG_0:0->7
        let tmp = read_phy(base_addr, 1892);
        write_phy(base_addr, 1892, tmp & 0xfffc0000 | 0x7);

        //PHY_PAD_ACS_IO_CFG:0->7
        let tmp = read_phy(base_addr, 1893);
        write_phy(base_addr, 1893, tmp & 0xfffc0000 | 0x7);

        //PHY_CAL_MODE_0 TODO
        let tmp = read_phy(base_addr, 1852);
        write_phy(base_addr, 1852, tmp & 0xffffe000 | 0x078);

        //PHY_PLL_WAIT
        let tmp = read_phy(base_addr, 1822);
        write_phy(base_addr, 1822, tmp | 0xFF);

        //PHY_PAD_VREF_CTRL_AC:10'h0100->10'h3d5
        let tmp = read_phy(base_addr, 1896);
        write_phy(base_addr, 1896, tmp & 0xfffffc00 | 0x03d5);

        //PHY_PAD_VREF_CTRL_DQ_x:10'h11f->10'h3d5
        let tmp = read_phy(base_addr, 91);
        write_phy(base_addr, 91, tmp & 0xfc00ffff | 0x03d50000);

        let tmp = read_phy(base_addr, 347);
        write_phy(base_addr, 347, tmp & 0xfc00ffff | 0x03d50000);

        let tmp = read_phy(base_addr, 603);
        write_phy(base_addr, 603, tmp & 0xfc00ffff | 0x03d50000);

        let tmp = read_phy(base_addr, 859);
        write_phy(base_addr, 859, tmp & 0xfc00ffff | 0x03d50000);

        // PHY_PAD_FDBK_DRIVE:bit[7:0]:{ENSLICEP_DRV,ENSLICEN_DRV}
        // SPEED[23:22]=3, tx_pulldwn[21:20]=2'b00, MODE[17:15]=7,
        // SLEWP[13:11]=7, SLEWN[10:8]=7
        write_phy(base_addr, 1912, 0xcc3bfc7);
        // PHY_PAD_FDBK_DRIVE2:bit[7:0]:{ENSLICEP_ODT,ENSLICEN_ODT}
        // BOOSTP[15:12]=F, BOOSTN[11:18]=F
        write_phy(base_addr, 1913, 0xff8f);
        // PHY_PAD_DATA_DRIVE
        // tx_pulldwn[21:20]=2'b11, SPEED[10:9]=3, MODE[8:6]=7,
        // SLEWP[5:3]=7, SLEWN[2:0]=7
        write_phy(base_addr, 1914, 0x33f07ff);
        // PHY_PAD_DQS_DRIVE
        // tx_pulldwn[13:12]=2'b11, SPEED[10:9]=3, MODE[8:6]=7,
        // SLEWP[5:3]=7, SLEWN[2:0]=7
        write_phy(base_addr, 1915, 0xc3c37ff);
        // PHY_PAD_ADDR_DRIVE
        // BOOSTP[26:23]=F, BOOSTN[22:19]=F, SLEWP[13:11]=7, SLEWN[10:8]=7,
        // tx_pulldwn[6:5]=2'b11
        write_phy(base_addr, 1916, 0x1fffff70);
        // PHY_PAD_ADDR_DRIVE2
        write_phy(base_addr, 1917, 0x230010);
        // PHY_PAD_CLK_DRIVE
        // BOOSTP[29:26]=F, BOOSTN[25:22]=F, tx_pulldwn[21:20]=2'b00,
        // SPEED[18:17]=3, MODE[16:14]=7, SLEWP[13:11]=7, SLEWN[10:8]=7
        write_phy(base_addr, 1918, 0x3fc7fff7);
        // PHY_PAD_CLK_DRIVE2
        write_phy(base_addr, 1919, 0xe10);
        // PHY_PAD_ERR_DRIVE
        // tx_pulldwn[28:27]=2'b11, BOOSTP[26:23]=F, BOOSTN[22:19]=F,
        // SPEED[18:17]=3, MODE[16:14]=7, SLEWP[13:11]=7, SLEWN[10:8]=7
        write_phy(base_addr, 1920, 0x1fffffff);
        // PHY_PAD_ERR_DRIVE2
        write_phy(base_addr, 1921, 0x188411);
        // PHY_PAD_CKE_DRIVE
        // tx_pulldwn[28:27]=2'b11, BOOSTP[26:23]=F, BOOSTN[22:19]=F,
        // SPEED[18:17]=3, MODE[16:14]=7, SLEWP[13:11]=7, SLEWN[10:8]=7
        write_phy(base_addr, 1922, 0x1fffffff);
        // PHY_PAD_CKE_DRIVE2
        write_phy(base_addr, 1923, 0x180400);
        // PHY_PAD_RST_DRIVE
        // tx_pulldwn[28:27]=2'b11, BOOSTP[26:23]=F, BOOSTN[22:19]=F,
        // SPEED[18:17]=3, MODE[16:14]=7, SLEWP[13:11]=7, SLEWN[10:8]=7
        write_phy(base_addr, 1924, 0x1fffffff);
        // PHY_PAD_RST_DRIVE2
        write_phy(base_addr, 1925, 0x180400);
        // PHY_PAD_CS_DRIVE
        // tx_pulldwn[28:27]=2'b11, BOOSTP[26:23]=F, BOOSTN[22:19]=F,
        // SPEED[18:17]=3, MODE[16:14]=7, SLEWP[13:11]=7, SLEWN[10:8]=7
        write_phy(base_addr, 1926, 0x1fffffcf);
        // PHY_PAD_CS_DRIVE2
        write_phy(base_addr, 1927, 0x188400);
        // PHY_PAD_ODT_DRIVE
        // tx_pulldwn[28:27]=2'b11, BOOSTP[26:23]=F, BOOSTN[22:19]=F,
        // SPEED[18:17]=3, MODE[16:14]=7, SLEWP[13:11]=7, SLEWN[10:8]=7
        write_phy(base_addr, 1928, 0x1fffffff);
        // PHY_CAL_CLK_SELECT_0,PHY_PAD_ODT_DRIVE2:1->4
        // PHY_CAL_CLK_SELECT_0[26:24]
        write_phy(base_addr, 1929, 0x4188411);

        //PHY_PAD_FDBK_TERM
        write_phy(base_addr, 1837, 0x24410);
        //PHY_PAD_ADDR_TERM
        write_phy(base_addr, 1840, 0x24410);
        //PHY_PAD_ERR_TERM
        write_phy(base_addr, 1842, 0x2ffff);

        /* ------------ */

        if ddr_num == 0 {
            // PHY_DQ_TSEL_SELECT_X
            // bit[15:8]={ENSLICEP_DRV,ENSLICEN_DRV}:tsel_wr_select
            // bit[7:0]={ENSLICEP_ODT,ENSLICEN_ODT}:tsel_rd_select
            let tmp = read_phy(base_addr, 76);
            write_phy(base_addr, 76, tmp & 0xff0000ff | 0x00ff8f00);

            let tmp = read_phy(base_addr, 332);
            write_phy(base_addr, 332, tmp & 0xff0000ff | 0x00ff7c00);

            let tmp = read_phy(base_addr, 588);
            write_phy(base_addr, 588, tmp & 0xff0000ff | 0x00ff8f00);

            let tmp = read_phy(base_addr, 844);
            write_phy(base_addr, 844, tmp & 0xff0000ff | 0x00ff8f00);

            // PHY_DQS_TSEL_SELECT_X
            // bit[15:8]={ENSLICEP_DRV,ENSLICEN_DRV}:tsel_wr_select
            // bit[7:0]={ENSLICEP_ODT,ENSLICEN_ODT}:tsel_rd_select
            let tmp = read_phy(base_addr, 77);
            write_phy(base_addr, 77, tmp & 0xffff0000 | 0xc47c);

            let tmp = read_phy(base_addr, 333);
            write_phy(base_addr, 333, tmp & 0xffff0000 | 0xc48f);

            let tmp = read_phy(base_addr, 589);
            write_phy(base_addr, 589, tmp & 0xffff0000 | 0xc47c);

            let tmp = read_phy(base_addr, 845);
            write_phy(base_addr, 845, tmp & 0xffff0000 | 0xc47c);
        } else {
            //ddr1
            let tmp = read_phy(base_addr, 76);
            write_phy(base_addr, 76, tmp & 0xff0000ff | 0x00ff4f00);

            let tmp = read_phy(base_addr, 332);
            write_phy(base_addr, 332, tmp & 0xff0000ff | 0x00ff7c00);

            let tmp = read_phy(base_addr, 588);
            write_phy(base_addr, 588, tmp & 0xff0000ff | 0x00ff4f00);

            let tmp = read_phy(base_addr, 844);
            write_phy(base_addr, 844, tmp & 0xff0000ff | 0x00ff7c00);

            let tmp = read_phy(base_addr, 77);
            write_phy(base_addr, 77, tmp & 0xffff0000 | 0xc48f);

            let tmp = read_phy(base_addr, 333);
            write_phy(base_addr, 333, tmp & 0xffff0000 | 0xc48f);

            let tmp = read_phy(base_addr, 589);
            write_phy(base_addr, 589, tmp & 0xffff0000 | 0xc48f);

            let tmp = read_phy(base_addr, 845);
            write_phy(base_addr, 845, tmp & 0xffff0000 | 0xc48f);
        }

        //PHY_ADR_TSEL_SELECT_X:bit[7:0]:{ENSLICEP_ODT/DRV,PENSLICEN_ODT/DRV}
        let tmp = read_phy(base_addr, 1062); // addr5-0
        write_phy(base_addr, 1062, tmp & 0xffffff00 | 0xff);

        let tmp = read_phy(base_addr, 1318); // addr11-6
        write_phy(base_addr, 1318, tmp & 0xffffff00 | 0xff);

        let tmp = read_phy(base_addr, 1574); // addr15-12
        write_phy(base_addr, 1574, tmp & 0xffffff00 | 0xff);

        //PHY_TST_CLK_PAD_CTRL_x
        write_phy(base_addr, 1848, 0x03cf_07f9);
        write_phy(base_addr, 1849, 0x0000_003f);
        write_phy(base_addr, 1850, 0x0007_0000);
        write_phy(base_addr, 1851, 0x0019_0000);

        //PHY_DSLICE_PAD_BOOSTPN_SETTING_x
        let tmp = read_phy(base_addr, 130);
        write_phy(base_addr, 130, tmp | 0x00ff_0000);
        let tmp = read_phy(base_addr, 386);
        write_phy(base_addr, 386, tmp | 0x00ff_0000);
        let tmp = read_phy(base_addr, 642);
        write_phy(base_addr, 642, tmp | 0x6600_0000);
        let tmp = read_phy(base_addr, 898);
        write_phy(base_addr, 898, tmp | 0x6600_0000);

        if true {
            //PHY_WRLVL_CAPTURE_CNT_X
            let tmp = read_phy(base_addr, 29);
            write_phy(base_addr, 29, (tmp & 0xc0ffffff) | 0x10000000);

            let tmp = read_phy(base_addr, 285);
            write_phy(base_addr, 285, (tmp & 0xc0ffffff) | 0x10000000);

            let tmp = read_phy(base_addr, 541);
            write_phy(base_addr, 541, (tmp & 0xc0ffffff) | 0x10000000);

            let tmp = read_phy(base_addr, 797);
            write_phy(base_addr, 797, (tmp & 0xc0ffffff) | 0x10000000);

            //PHY_GTLVL_CAPTURE_CNT_X
            let tmp = read_phy(base_addr, 30);
            write_phy(base_addr, 30, tmp | 0x00080000);

            let tmp = read_phy(base_addr, 286);
            write_phy(base_addr, 286, tmp | 0x00080000);

            let tmp = read_phy(base_addr, 542);
            write_phy(base_addr, 542, tmp | 0x00080000);

            let tmp = read_phy(base_addr, 798);
            write_phy(base_addr, 798, tmp | 0x00080000);

            //PHY_RDLVL_CAPTURE_CNT_X
            let tmp = read_phy(base_addr, 31);
            write_phy(base_addr, 31, (tmp & 0xFFFFFFC0) | 0x00000010);

            let tmp = read_phy(base_addr, 287);
            write_phy(base_addr, 287, (tmp & 0xFFFFFFC0) | 0x00000010);

            let tmp = read_phy(base_addr, 543);
            write_phy(base_addr, 543, (tmp & 0xFFFFFFC0) | 0x00000010);

            let tmp = read_phy(base_addr, 799);
            write_phy(base_addr, 799, (tmp & 0xFFFFFFC0) | 0x00000010);

            //PHY_ADRLVL_CAPTURE_CNT_X
            let tmp = read_phy(base_addr, 1071);
            write_phy(base_addr, 1071, (tmp & 0xFFFFFFF0) | 0x00000008);

            let tmp = read_phy(base_addr, 1327);
            write_phy(base_addr, 1327, (tmp & 0xFFFFFFF0) | 0x00000008);

            let tmp = read_phy(base_addr, 1583);
            write_phy(base_addr, 1583, (tmp & 0xFFFFFFF0) | 0x00000008);

            //PHY_CSLVL_COARSECAPTURE_CNT
            let tmp = read_phy(base_addr, 1808);
            write_phy(base_addr, 1808, (tmp & 0xFFFFFFF0) | 0x00000008);

            //PHY_CSLVL_CAPTURE_CNT_X
            let tmp = read_phy(base_addr, 1896);
            write_phy(base_addr, 1896, (tmp & 0xFFFFFFF0) | 0x00080000);
        }
    }
}
